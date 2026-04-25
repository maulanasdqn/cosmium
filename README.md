# cosmium

A patched Chromium variant that runs inside containers without being fingerprinted as one.

Cosmium is **not** a fork — it is a thin patch repository that pins a Chromium tag, applies a series of surgical C++ patches, and builds. The pattern is the same one used by Brave, Bromite, and ungoogled-chromium. Upstream security fixes come for free; the maintenance surface stays small.

This repo is consumed by [`mrscraper-rs`](../mrscraper-rs) — its `infrastructure/browser/` chromiumoxide pool just needs the path to a built `cosmium` binary.

---

## Why a custom build?

Generic stealth tools (patchright, undetected-chromedriver, puppeteer-extra-stealth) handle the *automation-API* tells: `navigator.webdriver`, CDP `Runtime.enable` leaks, AutomationControlled. They cannot fix the *hardware-absence* tells that betray containers, because those leaks happen at the C++ level before any user-script can run.

The container fingerprint surface cosmium targets:

| Tell | Why containers fail it |
|---|---|
| WebGL `UNMASKED_VENDOR_WEBGL` / `UNMASKED_RENDERER_WEBGL` | No GPU → SwiftShader strings |
| Client Hints (`Sec-CH-UA-Platform`, `-Arch`, `-Bitness`, `-Platform-Version`) | Auto-populated from real host OS — leaks Linux even with Windows UA |
| `navigator.mediaDevices.enumerateDevices()` | Returns `[]` — no audio/video hardware |
| `speechSynthesis.getVoices()` | Empty — no system TTS voices |
| `navigator.hardwareConcurrency` / `deviceMemory` | Low/uniform values, cgroup limits leak through |
| WebRTC STUN host candidates | Reveal Docker bridge IPs (`172.17.x.x`, `10.x.x.x`) |
| Canvas / WebGL pixel hashes | Software rendering produces SwiftShader-specific fingerprints |
| AudioContext `baseLatency` / `outputLatency` | No real audio device → distinct values |
| Timezone (`Intl.DateTimeFormat().resolvedOptions().timeZone`) | UTC by default; mismatches proxy geo |
| Screen `colorDepth` / DPR / `outerWidth/Height` | Xvfb defaults; DPR always 1.0 |
| Font enumeration | Minimal Linux container fonts (DejaVu only) |

---

## Architecture

Cosmium does not hardcode spoofed values. It reads a **profile** at startup:

```
cosmium --cosmium-profile=/profiles/win11_rtx3060_en-us.json <chrome flags>
```

The profile drives every spoofed surface in lockstep so values stay coherent (claimed-16-cores must match observed-CPU-timing, claimed-`en-US` must match Accept-Language must match Intl timezone, etc.). One mismatch and ML-based bot scoring catches you regardless of how many individual tells are clean.

C++ patches are the **hooks**; profiles are the **data**. See `profiles/schema.json` for the schema and `profiles/*.json` for examples.

### Variants

| Variant | When to use | Patches needed |
|---|---|---|
| `cosmium:gpu` | nvidia-docker host with `--gpus all` | Lighter — real GPU produces clean WebGL/canvas fingerprints; only need to spoof brand strings + nav properties |
| `cosmium:cpu` | Generic container hosts (no GPU) | Heavier — must fully spoof WebGL + canvas + audio at the rendering layer |

`cosmium:gpu` is the reference build. Start there.

### What this repo does NOT solve

- **Datacenter ASN detection.** Cloudflare, PerimeterX, Akamai score IP type (datacenter vs residential vs mobile) at the network layer. No browser patch fixes this. Run cosmium behind residential/mobile proxies (mrscraper-rs already exposes a proxy port).
- **Behavioral analysis.** Mouse curves, scroll rhythm, dwell time, click entropy. These belong in your automation layer (mrscraper-rs scenarios), not in the browser binary.
- **Captcha solving.** Out of scope. Use mrscraper-rs's captcha port adapters.

---

## Quickstart

Prerequisites: Linux build host (Debian 12 or Ubuntu 22.04 recommended), 100GB+ free disk, 16GB+ RAM, ~6h on a fast machine.

```bash
# One-time setup
./scripts/00-prereqs.sh           # installs depot_tools + Chromium build deps
./scripts/01-fetch.sh             # ~30 min, ~40GB
./scripts/02-checkout.sh          # checks out the pinned tag from VERSION

# Iterate
./scripts/03-apply-patches.sh     # applies patches/*.patch in series order
./scripts/04-build.sh             # ~4-8h first time; ~minutes incremental
./scripts/05-package.sh           # produces dist/cosmium-<version>.tar.zst

# Test
./scripts/test-fingerprint.sh dist/cosmium-*.tar.zst

# Reset (for rebasing patches against a new Chromium tag)
./scripts/reset.sh                # reverts all patches; src/ goes back to the pinned tag
```

---

## Layout

```
cosmium/
├── VERSION                 Chromium tag pin (e.g. "135.0.7049.84")
├── config/
│   ├── args.gn             gn build configuration (stealth-friendly defaults)
│   └── chromium.env        environment vars: CHROMIUM_TAG, build paths
├── scripts/                build orchestration (numbered for execution order)
├── patches/
│   ├── series              ordered list of patches to apply
│   ├── *.patch             actual patch files
│   └── design/             markdown design docs for unwritten patches
├── profiles/
│   ├── schema.json         JSON schema for fingerprint profiles
│   └── *.json              example profiles
└── docker/
    ├── Dockerfile.build    reproducible build container
    ├── Dockerfile.runtime.gpu
    ├── Dockerfile.runtime.cpu
    └── entrypoint.sh
```

---

## Status

Bootstrapping. The skeleton lands first; vanilla Chromium build is the next milestone. Patches arrive after that, one at a time, each validated against the fingerprint test harness before the next is written.
