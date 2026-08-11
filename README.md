# cosmium

A patched Chromium variant + Rust orchestration layer for stealth scraping inside containers.

Cosmium exists to solve one specific problem: a Chromium binary that does **not** fingerprint as a containerized environment. Generic stealth tooling (patchright, undetected-chromedriver) handles automation-API tells like `navigator.webdriver` but cannot fix the *hardware-absence* tells that betray Docker — SwiftShader WebGL strings, empty `mediaDevices`, leaked Linux Client Hints under a Windows UA, Docker bridge IPs in WebRTC candidates, UTC timezone, missing fonts.

This repo ships:

1. **A patch series** (`patches/*.patch`) against a pinned Chromium tag — surgical C++ edits that expose `--cosmium-*` command-line switches for every spoofable surface.
2. **A Rust workspace** (`apps/cli`, `apps/engine`, `.config`) that loads JSON fingerprint profiles, validates coherence, maps profile fields to Chromium switches, and launches the patched binary.
3. **An LLM-assisted profile authoring loop** via OpenRouter — generate, repair, and mutate coherent profiles from a persona description.
4. **Build + runtime Docker images** — reproducible Chromium build env (`docker/Dockerfile.build`) and GPU/CPU runtime images.

Cosmium ships a single binary that any CDP-capable client (chromiumoxide, puppeteer, playwright) can drive — point it at the built `chrome` executable like you would any other Chromium-based browser.

---

## Architecture

The Rust workspace follows a clean-architecture layout (domain → application → infrastructure → presentation, dependencies pointing only inward):

```
cosmium/
├── Cargo.toml                       workspace
├── rust-toolchain.toml              1.85 / edition 2024
├── VERSION                          pinned Chromium tag (135.0.7049.84)
│
├── .config/                         env + logger bootstrap shared by all binaries
│   └── src/{env.rs, logger.rs, lib.rs}
│
├── apps/
│   ├── cli/                         binary — wires logger and engine::cli::run()
│   │   └── src/main.rs
│   └── engine/                      library — domain / application / infrastructure / presentation
│       └── src/
│           ├── domain/
│           │   ├── profile/         Profile entity + value objects + ProfileRepository port
│           │   │   ├── identity.rs, locale.rs, hardware.rs, gpu.rs, screen.rs,
│           │   │   ├── audio.rs, media_devices.rs, voices.rs, fonts.rs,
│           │   │   ├── webrtc.rs, canvas_noise.rs, profile.rs, repository.rs
│           │   │   └── validation/  cross-field coherence rules + tests
│           │   ├── runtime/         BrowserRuntime port + profile_to_flags()
│           │   └── llm/             LlmClient port + ChatRequest/ChatResponse
│           ├── application/use_cases/
│           │   ├── validate_profile.rs
│           │   ├── list_profiles.rs
│           │   ├── run_browser.rs
│           │   ├── generate_profile.rs   LLM
│           │   ├── repair_profile.rs     LLM
│           │   └── mutate_profile.rs     LLM
│           ├── infrastructure/
│           │   ├── profile/         FsJsonProfileRepository
│           │   ├── runtime/         TokioProcessRuntime
│           │   └── llm/             OpenRouterClient
│           └── presentation/cli/    clap parser + command handlers
│
├── patches/                         Chromium C++ patches (apply via scripts/03-apply-patches.sh)
│   ├── series                       8 patches in order
│   ├── 0001-strip-navigator-webdriver.patch
│   ├── 0002-spoof-webgl-vendor-renderer.patch
│   ├── 0003-spoof-client-hints-platform.patch
│   ├── 0004-spoof-navigator-platform.patch
│   ├── 0005-spoof-navigator-languages.patch
│   ├── 0006-spoof-hardware-concurrency.patch
│   ├── 0007-spoof-device-memory.patch
│   ├── 0008-spoof-screen-color-depth.patch
│   └── design/                      design docs for deferred patches
│
├── profiles/                        coherent fingerprint specifications (JSON)
│   ├── schema.json                  authoritative profile shape
│   ├── win11_rtx3060_en-us.json
│   └── macos_m2_en-us.json
│
├── scripts/                         build orchestration (numbered 00-05)
│   ├── 00-prereqs.sh, 01-fetch.sh, 02-checkout.sh,
│   ├── 03-apply-patches.sh, 04-build.sh, 05-package.sh,
│   ├── reset.sh, make-patch.sh, test-fingerprint.sh, _lib.sh
│
└── docker/
    ├── Dockerfile.build             Debian build env with depot_tools + Chromium deps
    ├── Dockerfile.runtime.gpu       nvidia-docker runtime
    ├── Dockerfile.runtime.cpu       SwiftShader runtime
    ├── entrypoint.sh
    └── docker-compose.yml
```

---

## Patches

Eight patches landed against `135.0.7049.84`, authored against real source fetched via gitiles (not speculation).

| # | Switch | Vector closed |
|---|---|---|
| 0001 | _(unconditional)_ | `navigator.webdriver === false` always |
| 0002 | `--cosmium-webgl-vendor`, `--cosmium-webgl-renderer` | `UNMASKED_VENDOR_WEBGL` / `UNMASKED_RENDERER_WEBGL` — eliminates SwiftShader giveaway |
| 0003 | `--cosmium-ua-platform` | `navigator.userAgentData.platform` (JS-side Client Hints) |
| 0004 | `--cosmium-platform` | `navigator.platform` (Win32 / MacIntel / Linux x86_64) |
| 0005 | `--cosmium-languages` | `navigator.languages` array |
| 0006 | `--cosmium-hardware-concurrency` | `navigator.hardwareConcurrency` (cgroup leak fix) |
| 0007 | `--cosmium-device-memory` | `navigator.deviceMemory` |
| 0008 | `--cosmium-color-depth` | `screen.colorDepth` / `pixelDepth` |

Deferred (design docs in `patches/design/`):
- `0009-spoof-mediadevices-enumerate` — empty `enumerateDevices()` is the strongest container tell. Patch is more complex (Mojo IPC + GC objects); needs verification on the real tree before authoring.
- HTTP-level `Sec-CH-UA-Platform` header (separate code path from JS-side patch 0003).
- WebRTC host-candidate filter for Docker bridge IPs.
- Canvas / audio fingerprint per-profile noise.

The Rust runtime emits these switches automatically — see `apps/engine/src/domain/runtime/flags.rs`.

---

## Profiles

A profile is a coherent JSON document driving every spoofed surface. Coherence is enforced by `domain/profile/validation/` — cross-field rules JSON Schema cannot express:

- UA major Chrome version must equal `client_hints.brands` Chrome entry version
- `navigator.platform` must match `Sec-CH-UA-Platform` (Win32↔Windows, MacIntel↔macOS, Linux x86_64↔Linux)
- `locale.languages[0]` must be the prefix of `locale.accept_language`
- `locale.timezone` must be a real IANA zone
- `gpu.renderer` must NOT contain `SwiftShader` or `0x0000C0DE`
- `hardware.hardware_concurrency` must be even (real CPUs)
- `hardware.device_memory_gb` must be one of `[0.25, 0.5, 1, 2, 4, 8]` (Chrome's bucketing)
- `screen.color_depth` must equal `screen.pixel_depth`
- `media_devices` must contain at least one audioinput/audiooutput/videoinput
- `canvas_noise.seed` must be exactly 32 hex characters

Schema: [`profiles/schema.json`](profiles/schema.json). Examples: `profiles/win11_rtx3060_en-us.json`, `profiles/macos_m2_en-us.json`.

---

## CLI

```
cosmium profile validate <name|path>           # schema + coherence check
cosmium profile show <name|path>               # parsed JSON dump
cosmium profile list                           # enumerate profiles in COSMIUM_PROFILES_DIR
cosmium profile generate --persona "..." \     # LLM: persona → coherent profile
                         --name <slug> \
                         (--save | --output PATH)
cosmium profile repair <path>                  # LLM: fix coherence diagnostics
                       [--output PATH]
cosmium profile mutate <path> \                # LLM: produce N coherent variants
                       --count N \
                       [--hint "..."] \
                       (--save | --output-dir DIR)
cosmium run --profile <name|path> \            # launch patched chrome with mapped flags
            [--binary PATH] \
            [--flag <chrome-flag> ...] \
            [URL ...]
cosmium build [--from <phase>] [--only <phase>] # full Chromium build pipeline
              [--jobs N] [--install-build-deps]
              [--tag VERSION]
cosmium test fingerprint --profile <name|path>  # probe binary against profile, pass/fail report
                         [--binary PATH]
```

### LLM-assisted profile authoring (OpenRouter)

```bash
export OPENROUTER_API_KEY=sk-or-v1-...
# Optional:
# export OPENROUTER_MODEL=anthropic/claude-sonnet-4.6
# export OPENROUTER_BASE_URL=https://openrouter.ai/api/v1

# Generate from a persona
cosmium profile generate \
  --persona "Windows 11 gaming PC, RTX 4070, 16-core CPU, 32GB RAM, en-US, Central time" \
  --name win11_rtx4070_en-us \
  --save

# Produce 5 coherent variants for IP rotation
cosmium profile mutate profiles/win11_rtx3060_en-us.json --count 5 --save

# Vary along specific dimensions
cosmium profile mutate profiles/win11_rtx3060_en-us.json \
  --count 8 \
  --hint "vary primarily by locale and timezone, keep Windows + RTX-class GPU" \
  --save

# Fix a profile that fails coherence
cosmium profile repair profiles/some-broken.json
```

How it stays trustworthy:

- **Prompts embed ground truth.** `generate_profile.rs` includes `profiles/schema.json` and `profiles/win11_rtx3060_en-us.json` via `include_str!` — the LLM sees the exact schema and a known-good reference.
- **`response_format: json_object`.** OpenRouter forces strict JSON output for compatible models.
- **Generated profiles are validated.** Every output runs through the coherence validator before saving. Diagnostics are printed; warnings are advisory, errors block.
- **`repair` closes the loop.** Pass any failing profile + diagnostics back to the LLM; re-validate the result.
- **LLM is optional.** Validate, show, list, run all work without an API key. Only generate/repair/mutate require one.

---

## Build pipeline

Prerequisites: Linux build host (Docker on Windows works via WSL2 backend), 100GB+ disk, 16GB+ RAM, ~6h on a fast machine.

```bash
# Build the build container once (~5 min, 2 GB image)
docker build -f docker/Dockerfile.build -t cosmium-build:latest .

# Then run the full pipeline (~6h first time, minutes incremental)
docker compose -f docker/docker-compose.yml run --rm build \
  cargo run --release -p cosmium-cli -- build --install-build-deps

# Or run a single phase for iteration
docker compose -f docker/docker-compose.yml run --rm build \
  cargo run --release -p cosmium-cli -- build --only apply

# The legacy shell scripts (scripts/00-05.sh) still work and produce the same output —
# the Rust subcommand orchestrates the same git/gclient/gn/autoninja invocations.
```

Output: `dist/cosmium-<tag>.tar.zst` containing the `chrome` binary + ICU + locales + ANGLE.

The compose file persists the Chromium source tree, build outputs, and depot_tools in named Docker volumes — losing them costs ~30min refetch + several hours rebuild.

To rebase onto a newer Chromium tag: bump `VERSION`, run `./scripts/reset.sh`, then re-checkout / re-apply / re-build.

---

## Runtime

```bash
# Build the runtime image consuming the tarball
docker build -f docker/Dockerfile.runtime.gpu \
  --build-arg COSMIUM_TARBALL=dist/cosmium-135.0.7049.84.tar.zst \
  -t cosmium:gpu .

# Launch via the cosmium CLI
COSMIUM_BINARY=/opt/cosmium/chrome \
cosmium run --profile win11_rtx3060_en-us https://browserleaks.com/javascript

# Or directly
docker run --rm --gpus all --cap-add SYS_ADMIN \
  -v $PWD/profiles:/profiles:ro \
  -e COSMIUM_PROFILE_NAME=win11_rtx3060_en-us \
  cosmium:gpu https://browserleaks.com/javascript
```

### Fingerprint testing

```bash
# After a build, verify the binary matches the profile
cosmium test fingerprint \
  --profile win11_rtx3060_en-us \
  --binary out/cosmium/chrome
```

Probes that ship out of the box: `webdriver`, `platform`, `language`, `languages`,
`hardware_concurrency`, `device_memory`, `color_depth`, `ua_data_platform`,
`webgl_vendor`, `webgl_renderer`, `no_swiftshader`, `timezone`. Each probe's expected
value is derived from the loaded profile, so the report tells you exactly which
patches landed correctly and which are still broken.

### Profile → flags mapping

The `cosmium run` subcommand maps a profile to a flag list (verified by `domain/runtime/flags.rs::tests`):

```
--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36
--lang=en-US
--accept-lang=en-US,en;q=0.9
--cosmium-platform=Win32
--cosmium-ua-platform=Windows
--cosmium-languages=en-US,en
--cosmium-hardware-concurrency=12
--cosmium-device-memory=8
--cosmium-color-depth=24
--cosmium-webgl-vendor=Google Inc. (NVIDIA)
--cosmium-webgl-renderer=ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 ...)
--window-size=1920,1080
--force-device-scale-factor=1
--force-webrtc-ip-handling-policy=default_public_interface_only
--disable-blink-features=AutomationControlled
--disable-features=Translate,InterestFeedContentSuggestions
--no-default-browser-check
--no-first-run
```

---

## Environment

| Variable | Default | Purpose |
|---|---|---|
| `COSMIUM_ROOT` | `$PWD` | repo root used to resolve other paths |
| `COSMIUM_PROFILES_DIR` | `$COSMIUM_ROOT/profiles` | where `FsJsonProfileRepository` reads |
| `COSMIUM_PATCHES_DIR` | `$COSMIUM_ROOT/patches` | (reserved) |
| `COSMIUM_BUILD_OUT` | `$COSMIUM_ROOT/out/cosmium` | gn out directory |
| `COSMIUM_BINARY` | `$COSMIUM_BUILD_OUT/chrome` | path the `run` subcommand launches |
| `COSMIUM_LOG` | `info,cosmium=debug,engine=debug` | tracing filter |
| `OPENROUTER_API_KEY` | _(unset)_ | required only for generate/repair/mutate |
| `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` | |
| `OPENROUTER_MODEL` | `anthropic/claude-sonnet-4.6` | any OpenRouter model with JSON mode |
| `OPENROUTER_REFERER` | _(unset)_ | optional `HTTP-Referer` header |
| `OPENROUTER_TITLE` | _(unset)_ | optional `X-Title` header |

A `.env` at repo root is auto-loaded.

---

## Embedding in your own Rust project

The engine crate exposes a clean port-and-adapter API. Add it as a path or git dependency:

```toml
[dependencies]
cosmium-engine = { git = "https://github.com/<your-org>/cosmium", package = "engine" }
```

Then map a profile to launch flags from your own code:

```rust
use cosmium_engine::domain::runtime::profile_to_flags;
use cosmium_engine::infrastructure::profile::FsJsonProfileRepository;

let repo = FsJsonProfileRepository::new("./profiles");
let profile = repo.load(std::path::Path::new("win11_rtx3060_en-us")).await?;
let flags = profile_to_flags(&profile);
// hand `flags` to chromiumoxide / fantoccini / your own CDP client
```

---

## What this repo does NOT solve

- **Datacenter ASN detection.** Anti-bot vendors score IP type (datacenter vs residential vs mobile) at the network layer. No browser patch fixes this — run cosmium behind a residential or mobile proxy.
- **Behavioral analysis.** Mouse curves, scroll rhythm, dwell time, click entropy. These belong in your automation layer, not in the browser binary.
- **Captcha solving.** Out of scope. Plug a captcha solver into your own automation layer.

---

## Status

Bootstrapping — Rust workspace is functional and tested; patches are authored against real Chromium source but not yet compile-tested against a fetched tree. Next milestone: trigger the long Docker build and verify `03-apply-patches.sh` applies the series cleanly. After that: tackle the deferred mediaDevices patch and the HTTP-level Client Hints patch.

Tests: `cargo test --workspace` (7 passing — coherence rules + flag mapper).
Build: `cargo build` (clean).
Code style: zero comments, max 200 LOC per file.

---

## Crates

Available on [crates.io](https://crates.io):

| Crate | |
|-------|-|
| [`cosmium-cli`](https://crates.io/crates/cosmium-cli) | [![crates.io](https://img.shields.io/crates/v/cosmium-cli)](https://crates.io/crates/cosmium-cli) |
| [`cosmium-engine`](https://crates.io/crates/cosmium-engine) | [![crates.io](https://img.shields.io/crates/v/cosmium-engine)](https://crates.io/crates/cosmium-engine) |
| [`cosmium-config`](https://crates.io/crates/cosmium-config) | [![crates.io](https://img.shields.io/crates/v/cosmium-config)](https://crates.io/crates/cosmium-config) |

```sh
cargo install cosmium-cli
```

## License

MIT
