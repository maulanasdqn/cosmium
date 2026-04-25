# Profiles

A profile is a JSON document that drives every spoofed surface in cosmium *coherently*. Each browser instance loads exactly one profile via:

```
cosmium --cosmium-profile=/profiles/win11_rtx3060_en-us.json
```

## Why coherence matters

If `navigator.hardwareConcurrency = 16` but a CPU benchmark (`crypto.subtle.deriveKey` timing, e.g.) finishes in 4-core time, you fail. If `navigator.platform = "Win32"` but `Sec-CH-UA-Platform` says `"Linux"`, you fail. If `navigator.languages = ["en-US"]` but `Intl.DateTimeFormat().resolvedOptions().timeZone = "Asia/Tokyo"`, you fail.

Anti-bot ML scoring catches the *correlation* between fields, not just their individual values. The profile system enforces consistency by making every field configurable from one source of truth.

## Schema

See [`schema.json`](./schema.json) for the canonical structure. Top-level groups:

| Group | Drives |
|---|---|
| `identity` | UA, ClientHints, `navigator.platform`, `navigator.appVersion` |
| `locale` | `navigator.languages`, `Accept-Language`, timezone, currency |
| `hardware` | CPU cores, RAM, max touch points, battery |
| `gpu` | WebGL vendor/renderer, WebGPU adapter info |
| `screen` | resolution, color depth, DPR, viewport |
| `audio` | AudioContext sample rate, latency, output devices |
| `media_devices` | enumerateDevices() return values |
| `voices` | speechSynthesis.getVoices() return values |
| `fonts` | font enumeration whitelist |
| `webrtc` | STUN policy, host candidate behavior |
| `canvas_noise` | per-profile deterministic canvas/audio noise seed |

## Authoring a new profile

1. Pick a target real-world device (Windows 11 + RTX 3060, macOS M2, Pixel 7, etc.).
2. Capture its real fingerprint by visiting [creepjs](https://abrahamjuliot.github.io/creepjs/), [browserleaks](https://browserleaks.com/), [bot.sannysoft.com](https://bot.sannysoft.com/) on the target device with vanilla Chrome and copying every probed value.
3. Fill in the profile JSON — every field. Missing fields fall back to whatever Chromium would emit by default, which usually re-introduces a tell.
4. Validate: `./scripts/validate-profile.sh profiles/your-profile.json`
5. Test: run cosmium with the profile and the fingerprint test harness (`./scripts/test-fingerprint.sh`).

## Examples shipped with cosmium

| File | Persona |
|---|---|
| `win11_rtx3060_en-us.json` | Windows 11 desktop, gaming PC, US English |
| `macos_m2_en-us.json` | MacBook Air M2, US English |
| `win10_intel_uhd_pt-br.json` | Office laptop, integrated GPU, Brazilian Portuguese |

Rotate profiles per-session in your automation pool by passing different `--cosmium-profile` paths to each worker.

## Anti-patterns

- **Don't randomize within a session.** Sites checksum fingerprint stability across requests; values that drift between page loads are themselves a tell.
- **Don't reuse one profile across thousands of IPs.** Anti-bot vendors share fingerprint databases; a profile + IP-range correlation that's seen too often gets baked into deny-lists.
- **Don't pick exotic devices.** A profile claiming "Lenovo ThinkStation P920 with dual Xeon Platinum 8280s" is a 0.001%-of-population device that itself looks suspicious. Pick boring, common hardware.
