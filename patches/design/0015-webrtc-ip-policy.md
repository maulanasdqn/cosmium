# patch 0015-webrtc-ip-policy

WebRTC's STUN candidate gathering reveals the real network identity behind any proxy or VPN. In a Docker container, the host candidates expose Docker bridge addresses (`172.17.x.x`, `10.x.x.x`) — instant container tell.

## Detection vector

```js
const pc = new RTCPeerConnection({ iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] });
pc.createDataChannel('');
pc.createOffer().then(o => pc.setLocalDescription(o));
pc.onicecandidate = e => {
  if (e.candidate) {
    // typ host candidates leak local IPs
    // 172.17.x.x → Docker bridge
    // 10.x.x.x   → corporate/VPN/Docker overlay
    // 192.168.x.x → home LAN (passes as legit)
    console.log(e.candidate.candidate);
  }
};
```

Modern Chrome (M85+) emits mDNS-obfuscated host candidates by default (`abc-def.local`), which fixes the most obvious leak. But:

- `srflx` (server-reflexive) candidates still reveal the public IP of the *real* network, bypassing any HTTP proxy.
- mDNS obfuscation is itself a fingerprintable behavior — sites probe whether `.local` candidates resolve.
- Pre-M85 sites still expect raw IPs and probe behavior.

## Target files

| File | Change |
|---|---|
| `chrome/browser/media/webrtc/webrtc_log_uploader.cc` | (verify path; ip handling policy lives near here) |
| `services/network/p2p/socket_manager.cc` | Gating of host candidate gathering by `IPHandlingPolicy` |
| `third_party/blink/renderer/modules/peerconnection/rtc_peer_connection.cc` | Apply policy at PC construction |
| `chrome/browser/media/webrtc/webrtc_local_ips_allowed_urls_policy_handler.cc` | Force-disable any allow-list overrides |
| Possibly `third_party/webrtc/p2p/base/port_allocator.cc` (vendored WebRTC) | Filter out Docker bridge ranges (`172.16-31.x.x`, `10.x.x.x`) at the candidate-source level |

## Profile fields read

```
webrtc.ip_handling_policy → enum: default | default_public_interface_only |
                                  default_public_and_private_interfaces |
                                  disable_non_proxied_udp
webrtc.stun_servers       → optional: which STUN servers PCs may contact
                            (empty array → block STUN entirely)
```

## Implementation notes

- **Docker bridge filter.** Beyond the policy enum, add a hardcoded filter that drops any candidate whose IP falls in RFC1918 ranges associated with container networks (`172.17.0.0/12`, `10.0.0.0/8` partial). Real residential users almost never have these on local interfaces; corporate users do, but their browser fingerprint also matches a corporate persona. Containers are the worst case.
- **mDNS obfuscation toggle.** Real Chrome enables mDNS hostnames for host candidates by default. Don't disable this — sites expect the `.local` behavior.
- **`disable_non_proxied_udp`** is the strongest setting: WebRTC only operates through the configured proxy. Combined with a proxy in front of cosmium, this means STUN never leaves the proxy, which means no IP leak at all.
- **`getStats()` leaks too.** `RTCPeerConnection.getStats()` returns `local-candidate` records with the actual IPs even if `onicecandidate` was suppressed. Override the stats-collection path or sanitize values.

## Validation

```js
const pc = new RTCPeerConnection({ iceServers: [{ urls: 'stun:stun.l.google.com:19302' }] });
pc.createDataChannel('');
pc.createOffer().then(o => pc.setLocalDescription(o));
pc.onicecandidate = e => e.candidate && console.log(e.candidate.candidate);
setTimeout(() => {
  pc.getStats().then(stats => {
    stats.forEach(r => {
      if (r.type === 'local-candidate') console.log('stat:', r.ip, r.candidateType);
    });
  });
}, 2000);
```

Expected output: only `srflx` candidates with the proxy's public IP, or `.local` mDNS hosts. **No** `172.x.x.x` / `10.x.x.x` host candidates anywhere.

Sites: https://browserleaks.com/webrtc, https://www.expressvpn.com/webrtc-leak-test.
