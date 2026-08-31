---
title: Installation
description: Install the Cosmium CLI from crates.io or from source, and obtain a patched Chromium binary.
---

Cosmium has two pieces to install: the **CLI** (fast, a normal Rust build) and
the **patched Chromium binary** (slow, a full Chromium build). You can do useful
work with only the first.

## The CLI

```bash
cargo install cosmium-cli
```

This gives you the `cosmium` binary. Requires Rust 1.85 or newer — the workspace
targets edition 2024.

Published crates:

| Crate | Purpose |
| --- | --- |
| [`cosmium-cli`](https://crates.io/crates/cosmium-cli) | the `cosmium` binary |
| [`cosmium-engine`](https://crates.io/crates/cosmium-engine) | the library, for embedding |
| [`cosmium-config`](https://crates.io/crates/cosmium-config) | env + logger bootstrap |

### From source

Building from source is required if you want `cosmium serve`, which is not
included in every published release:

```bash
git clone https://github.com/maulanasdqn/cosmium
cd cosmium
cargo build --release -p cosmium-cli
# binary at ./target/release/cosmium
```

## What works without a browser

These subcommands are pure Rust and need no Chromium binary:

- `cosmium profile validate` / `show` / `list`
- `cosmium profile generate` / `repair` / `mutate` (needs an OpenRouter key)
- `cosmium build` — it *produces* the binary

Everything under `cosmium run`, `cosmium scrape`, and `cosmium test` launches a
browser and needs one.

## The patched Chromium binary

There is no prebuilt download; the binary is built from the pinned Chromium tag
with the patch series applied. Budget **100 GB of disk, 16 GB of RAM, and around
six hours** for a first build on a fast machine.

```bash
# Build the build container once (~5 min, 2 GB image)
docker build -f docker/Dockerfile.build -t cosmium-build:latest .

# Run the full pipeline
docker compose -f docker/docker-compose.yml run --rm build \
  cargo run --release -p cosmium-cli -- build --install-build-deps
```

Output lands in `dist/cosmium-<tag>.tar.zst`, containing the `chrome` binary
plus ICU, locales, and ANGLE. The [building guide](/operations/building/) covers
phases, incremental rebuilds, and rebasing onto a newer Chromium tag.

## Point the CLI at the binary

Every browser-launching subcommand resolves the binary in this order:

1. the `--binary` flag
2. the `COSMIUM_BINARY` environment variable
3. `$COSMIUM_ROOT/out/cosmium/chrome`

```bash
export COSMIUM_BINARY=/opt/cosmium/chrome
cosmium run --profile win11_rtx3060_en-us https://browserleaks.com/javascript
```

## Profiles

Profile lookup uses `COSMIUM_PROFILES_DIR`, defaulting to
`$COSMIUM_ROOT/profiles`. If you installed via `cargo install`, that directory
does not exist yet — either clone the repo for its reference profiles, or point
the variable somewhere of your own:

```bash
export COSMIUM_PROFILES_DIR=~/.config/cosmium/profiles
```

A `.env` file at the repo root is auto-loaded, so local development usually
needs no exported variables at all. See [environment](/reference/environment/)
for every variable.

## Verify the install

```bash
cosmium --version
cosmium profile list
cosmium profile validate win11_rtx3060_en-us
```

With a browser binary available, the real check is the probe suite:

```bash
cosmium test fingerprint --profile win11_rtx3060_en-us
```

It reports pass/fail per spoofed surface, so you learn exactly which patches
landed. See [testing stealth](/guides/testing/).
