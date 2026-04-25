# patch 0003-spoof-client-hints

The single most-overlooked container tell. UA can be spoofed via `--user-agent` and CDP overrides. Client Hints cannot — they're auto-generated from the actual host OS. A Linux container with `User-Agent: ...Windows NT 10.0...` happily sends `Sec-CH-UA-Platform: "Linux"` as a request header AND populates `navigator.userAgentData.platform = "Linux"` in JS.

## Detection vector

```js
const ua = await navigator.userAgentData.getHighEntropyValues([
  'platform', 'platformVersion', 'architecture', 'bitness', 'model', 'wow64'
]);
// In a Linux container claiming Windows UA:
//   ua.platform === "Linux"               ← BUSTED
//   ua.architecture === "x86" or whatever the host actually is
```

Server-side: anti-bot checks `Sec-CH-UA-Platform` header against UA string consistency. Mismatch → score-up.

## Target files

| File | Change |
|---|---|
| `services/network/public/cpp/client_hints.cc` | The `BuildUserAgentMetadata` / `GetClientHintHeaderValue` paths — read profile values for platform, platform_version, architecture, bitness, model, mobile, wow64 |
| `third_party/blink/renderer/core/frame/navigator_ua_data.cc` | `NavigatorUAData::getHighEntropyValues` — return profile values |
| `content/browser/client_hints/client_hints.cc` | Browser-process-side header injection consults profile |
| `services/network/url_loader.cc` (or related) | If headers are added at network layer, override there too |
| `third_party/blink/renderer/core/loader/document_loader.cc` | If headers are stamped at document load, override |

## Profile fields read

```
identity.client_hints.brands         → Sec-CH-UA, navigator.userAgentData.brands
identity.client_hints.platform       → Sec-CH-UA-Platform, .platform
identity.client_hints.platform_version → Sec-CH-UA-Platform-Version, .platformVersion
identity.client_hints.architecture   → Sec-CH-UA-Arch, .architecture
identity.client_hints.bitness        → Sec-CH-UA-Bitness, .bitness
identity.client_hints.model          → Sec-CH-UA-Model, .model
identity.client_hints.mobile         → Sec-CH-UA-Mobile, .mobile
identity.client_hints.wow64          → .wow64 (no header)
identity.user_agent                  → User-Agent header
```

## Implementation notes

- **Two layers, one source of truth.** Client Hints are emitted both as HTTP request headers (network layer) AND as JS API values (`navigator.userAgentData`). Patch both, otherwise a server probe and a JS probe disagree — itself a tell.
- **High-entropy hints are gated.** `getHighEntropyValues()` requires the site to opt-in via Permissions Policy. The low-entropy default trio (`Sec-CH-UA`, `Sec-CH-UA-Mobile`, `Sec-CH-UA-Platform`) is sent unconditionally. Patch the unconditional path first.
- **Brand list "GREASE" entries.** Real Chrome includes a randomized `"Not.A/Brand"`-style entry that varies. Either pin one in the profile (predictable, possibly trackable across sessions) or generate one deterministically from the profile name's hash (recommended).
- **`Critical-CH` / `Accept-CH` server caching.** Some servers cache CH values per-IP. Make sure profile changes don't surprise the server with a different CH set on a previously-seen connection — that's only a problem if you reuse profiles across IPs, which the README already warns against.

## Validation

```js
const ua = await navigator.userAgentData.getHighEntropyValues([
  'platform','platformVersion','architecture','bitness','model','wow64','fullVersionList'
]);
console.log(JSON.stringify(ua, null, 2));
```

Plus server-side: hit a request inspector (e.g. https://webhook.site) and verify all `Sec-CH-UA-*` headers match the profile.
