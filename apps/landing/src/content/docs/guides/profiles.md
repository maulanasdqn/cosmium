---
title: Fingerprint profiles
description: The profile model, the coherence rules Cosmium enforces, and how to author a new one.
---

A profile is a single JSON document describing one coherent machine. Every
spoofable surface in the patched Chromium reads from it, so the profile — not
the code — is where a fingerprint is decided.

## The shape

Twelve required top-level sections, defined authoritatively in
`profiles/schema.json`:

| Section | Covers |
| --- | --- |
| `identity` | user agent, Client Hints, `navigator.platform`, appVersion |
| `locale` | languages, `accept_language`, IANA timezone |
| `hardware` | CPU count, device memory, touch points, optional battery |
| `gpu` | WebGL vendor and renderer strings |
| `screen` | width/height, avail box, color and pixel depth, DPR |
| `audio` | sample rate, base and output latency, max channels |
| `media_devices` | the `enumerateDevices()` list |
| `voices` | the `speechSynthesis.getVoices()` list |
| `fonts` | installed font list |
| `webrtc` | IP handling policy |
| `canvas_noise` | 32-hex-character per-profile noise seed |
| `name` | the profile's own identifier |

Two optional top-level fields matter a lot:

- **`chrome_version`** — when set, rewrites the UA's `Chrome/x.y.z.w` to
  `Chrome/<major>.0.0.0` and emits `--cosmium-chrome-version`. This is how you
  present a newer Chrome than the binary you built.
- **`strip_automation_tells`** — opt-in, emits
  `--cosmium-strip-automation-tells`. Not on by default.

## Coherence rules

Schema validity is not enough. A document can satisfy every type constraint and
still describe a machine that cannot exist — and an impossible machine is a
*stronger* detection signal than an unspoofed container.

`domain/profile/validation/` enforces the cross-field rules:

- The UA's major Chrome version must equal the Client Hints Chrome brand
  version. Disagreement between `navigator.userAgent` and
  `navigator.userAgentData.brands` is trivially checked and rarely benign.
- `navigator.platform` must match `Sec-CH-UA-Platform`: `Win32`↔`Windows`,
  `MacIntel`↔`macOS`, `Linux x86_64`↔`Linux`.
- `locale.languages[0]` must be a prefix of `locale.accept_language`.
- `locale.timezone` must be a real IANA zone name.
- `gpu.renderer` must not contain `SwiftShader` or `0x0000C0DE` — the two
  strings that mean "software rendering, therefore probably a container".
- `hardware.hardware_concurrency` must be even. Real consumer CPUs report even
  thread counts; a cgroup-limited container often does not.
- `hardware.device_memory_gb` must be one of `0.25, 0.5, 1, 2, 4, 8` — Chrome
  buckets this value, so any other number is impossible.
- `screen.color_depth` must equal `screen.pixel_depth`.
- `media_devices` must contain at least one `audioinput`, one `audiooutput`,
  and one `videoinput`. An empty device list is the single strongest container
  tell there is.
- `canvas_noise.seed` must be exactly 32 hexadecimal characters.

## Validating

```bash
cosmium profile validate win11_rtx3060_en-us
cosmium profile validate ./my-profile.json --strict
```

Diagnostics carry a severity. Errors block; warnings are advisory unless you
pass `--strict`, which promotes them. Use `--strict` in CI.

## Authoring a new profile

Three routes, in increasing order of effort:

**Mutate an existing one.** Cheapest way to get a fleet of related but distinct
fingerprints for rotation:

```bash
cosmium profile mutate profiles/win11_rtx3060_en-us.json --count 5 --save
```

**Generate from a persona.** Describe the machine in prose:

```bash
cosmium profile generate \
  --persona "Windows 11 gaming PC, RTX 4070, 16-core CPU, 32GB RAM, en-US, Central time" \
  --name win11_rtx4070_en-us \
  --save
```

Both require an OpenRouter key — see
[LLM profile authoring](/guides/llm-authoring/).

**Write it by hand.** Copy `profiles/win11_rtx3060_en-us.json`, edit, and
validate. The reference profiles are the best documentation of realistic values;
`schema.json` carries per-field descriptions and examples.

## A note on fleets

If you rotate profiles across requests, rotate them *with* their proxies. A
profile that appears from three continents in ten minutes is a correlation
signal no amount of per-surface accuracy will cover. Keep a profile pinned to an
egress the way a real machine is pinned to a home network.
