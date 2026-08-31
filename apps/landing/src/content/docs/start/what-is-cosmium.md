---
title: What is Cosmium
description: The detection vectors Cosmium closes in Chromium's C++, and the ones it deliberately leaves to your automation layer.
---

Cosmium is a **patched Chromium variant plus a Rust orchestration layer**, built
to solve one specific problem: a browser binary that does not fingerprint as a
containerized environment.

## Why patch the browser

Stealth tooling generally works by injecting JavaScript before page scripts run
— redefining `navigator.webdriver`, overriding `getParameter` on the WebGL
context, patching `Intl.DateTimeFormat`. That approach handles automation-API
tells, but it has two structural weaknesses:

1. **The overrides are detectable.** A property redefined from JavaScript has a
   different descriptor, a different `toString()`, and a different prototype
   position than the native one. Detection scripts check exactly this.
2. **It does not cover every realm.** A page can re-ask the same question inside
   a Web Worker, a fresh iframe, or an `about:blank` document — contexts your
   injection may never reach. Cosmium's own probe suite includes a
   `webdriver_worker` check for precisely this reason.

Patching the C++ means the spoofed value **is** the native value. There is no
descriptor mismatch to find, and every realm inherits it.

## The container tells

These are the signals that betray Docker specifically, and what Cosmium does
about each:

| Tell | Default in a container | Cosmium's answer |
| --- | --- | --- |
| WebGL renderer | `SwiftShader` / `0x0000C0DE` | `--cosmium-webgl-vendor` / `-renderer` (patch 0002) |
| Client Hints platform | `Linux` under a Windows UA | patches 0003, 0011, 0025 |
| `navigator.platform` | `Linux x86_64` | `--cosmium-platform` (patch 0004) |
| CPU count | cgroup-limited, often odd | `--cosmium-hardware-concurrency` (0006) |
| Device memory | host value, unbucketed | `--cosmium-device-memory` (0007) |
| Timezone | UTC | `--cosmium-timezone` (0016, 0027) |
| Screen | headless 800×600 | `--cosmium-screen-*` (0028) |
| Canvas / audio | pixel-identical across runs | per-profile noise (0019, 0020) |
| Battery API | absent | `--cosmium-battery-*` (0022) |
| WebRTC candidates | Docker bridge IPs | public-interface-only default (0015) |

The full list, with the switch each patch introduces, is on the
[patch series](/reference/patches/) page.

## Coherence is the hard part

Spoofing each surface individually is not enough — the values have to agree with
each other. A profile claiming `MacIntel` with an NVIDIA RTX renderer is a
stronger signal than the SwiftShader string it replaced.

Cosmium models a fingerprint as a single JSON document and enforces cross-field
rules that JSON Schema cannot express:

- the UA's major Chrome version must equal the Client Hints brand version
- `navigator.platform` must match `Sec-CH-UA-Platform` (Win32↔Windows,
  MacIntel↔macOS, Linux x86_64↔Linux)
- `locale.languages[0]` must be a prefix of `locale.accept_language`
- `locale.timezone` must be a real IANA zone
- `gpu.renderer` must not contain `SwiftShader` or `0x0000C0DE`
- `hardware.hardware_concurrency` must be even — real CPUs are
- `hardware.device_memory_gb` must be one of Chrome's buckets:
  `0.25, 0.5, 1, 2, 4, 8`
- `screen.color_depth` must equal `screen.pixel_depth`
- `canvas_noise.seed` must be exactly 32 hex characters

See [fingerprint profiles](/guides/profiles/) for the full model.

## What Cosmium does not solve

Being explicit about the boundary matters more than the feature list:

- **Datacenter ASN detection.** Anti-bot vendors score IP reputation at the
  network layer. No browser patch changes your IP — run Cosmium behind a
  residential or mobile proxy. The [proxy rotation](/guides/proxies/) guide
  covers the pool, not the sourcing.
- **Behavioral analysis.** Mouse curves, scroll rhythm, dwell time, click
  entropy. These belong in your automation layer.
- **Captcha solving.** Out of scope. Plug in your own solver.

## What ships in the repo

1. **`patches/`** — 28 patches against Chromium `135.0.7049.84`, applied in
   `series` order.
2. **A Rust workspace** — `apps/cli` (binary), `apps/engine` (library),
   `.config` (env + logging), published to crates.io as `cosmium-cli`,
   `cosmium-engine`, and `cosmium-config`.
3. **`profiles/`** — the authoritative `schema.json` plus two reference
   profiles.
4. **`docker/`** — a reproducible Chromium build environment and GPU/CPU
   runtime images.
5. **`apps/ui`** — a React dashboard that talks to the HTTP server mode.
