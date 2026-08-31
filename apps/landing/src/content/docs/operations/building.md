---
title: Building Chromium
description: The six-phase build pipeline, incremental rebuilds, and rebasing onto a new Chromium tag.
---

Building the patched binary is the long pole: roughly **six hours** on a fast
machine for a cold build, minutes for an incremental one.

## Prerequisites

- A **Linux** build host. Docker on Windows works through the WSL2 backend.
- **100 GB+** free disk — the Chromium source tree alone is enormous.
- **16 GB+** RAM.

## The pipeline

`cosmium build` orchestrates six phases, in this order:

| Phase | Does |
| --- | --- |
| `prereqs` | install build dependencies |
| `fetch` | fetch depot_tools and the Chromium source |
| `checkout` | check out the tag pinned in `VERSION` |
| `apply` | apply `patches/series` in order |
| `compile` | gn gen + autoninja |
| `package` | produce `dist/cosmium-<tag>.tar.zst` |

The legacy shell scripts (`scripts/00-prereqs.sh` through `05-package.sh`) still
work and drive the same git, gclient, gn, and autoninja invocations.

## Running it

Build the container once:

```bash
docker build -f docker/Dockerfile.build -t cosmium-build:latest .
```

Then the full pipeline:

```bash
docker compose -f docker/docker-compose.yml run --rm build \
  cargo run --release -p cosmium-cli -- build --install-build-deps
```

### Single phases

Iterating on a patch means re-running `apply` and `compile` only:

```bash
# just re-apply the series
cosmium build --only apply

# from apply onward — apply, compile, package
cosmium build --from apply
```

`--only` and `--from` conflict; pass one. With neither, all six phases run.

### Other flags

```bash
cosmium build --jobs 32            # cap parallelism
cosmium build --tag 136.0.7103.60  # override VERSION for one run
```

## Output

`dist/cosmium-<tag>.tar.zst`, containing the `chrome` binary plus ICU data,
locales, and ANGLE. Point `COSMIUM_BINARY` at the extracted `chrome`, or feed
the tarball to a runtime image — see [Docker](/operations/docker/).

## Protect the volumes

The compose file keeps the Chromium source tree, build output, depot_tools, and
a git cache in named volumes:

```
cosmium-src  cosmium-out  cosmium-depot  cosmium-cache
```

Losing them costs about thirty minutes of refetch plus several hours of rebuild.
Do not `docker compose down -v` casually.

## Rebasing onto a newer Chromium tag

```bash
# 1. bump the pin
echo "136.0.7103.60" > VERSION

# 2. clean the tree
./scripts/reset.sh

# 3. check out the new tag
./scripts/02-checkout.sh

# 4. apply the series — conflicts surface here
./scripts/03-apply-patches.sh
```

Because each patch targets one vector in one named file, a conflict tells you
precisely which surface Chromium moved. Fix the patch, re-run
`make-patch.sh`, and confirm it reapplies from a clean tree before rebuilding.

After a successful rebuild, the probe suite is the real acceptance test:

```bash
cosmium test fingerprint --profile win11_rtx3060_en-us --binary out/cosmium/chrome
```

It exits non-zero on any failure, so it drops into a build gate directly. If it
fails broadly with no obvious pattern, check that patch 0023 applied — see
[patch series](/reference/patches/#patch-0023-is-load-bearing).

## Build configuration

The gn args file is selected at build time from the repo root. Release builds
use thin LTO, a single codegen unit, and stripped symbols for the Rust side;
the Chromium side follows the checked-in gn configuration.
