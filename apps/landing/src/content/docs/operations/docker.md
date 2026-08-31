---
title: Docker
description: The build image, the GPU and CPU runtime images, and the environment the entrypoint reads.
---

Three images ship in `docker/`: one for building Chromium, and two for running
it.

| Dockerfile | Image | For |
| --- | --- | --- |
| `Dockerfile.build` | `cosmium-build` | the Chromium build environment |
| `Dockerfile.runtime.gpu` | `cosmium:gpu` | nvidia-docker, real GPU rendering |
| `Dockerfile.runtime.cpu` | `cosmium:cpu` | SwiftShader fallback |

## GPU or CPU

This choice is a fingerprinting decision, not just a performance one.

The **GPU image** passes a real NVIDIA device through, so WebGL, canvas, and
WebGPU produce genuine hardware output. The only spoofing needed is the brand
strings — pixel-level fingerprints are already clean, because they come from
real silicon.

The **CPU image** renders through SwiftShader. Cosmium replaces the giveaway
renderer *string*, but the pixels it produces still come from a software
rasterizer, and a determined fingerprinter comparing rendered output against a
known-GPU corpus can tell. Use it when no GPU is available, and expect the
canvas noise patches (0019, 0020) to be carrying more weight.

## Building the runtime image

The runtime images consume a packaged tarball from `dist/`:

```bash
docker build -f docker/Dockerfile.runtime.gpu \
  --build-arg COSMIUM_TARBALL=dist/cosmium-135.0.7049.84.tar.zst \
  -t cosmium:gpu .
```

## Running

```bash
docker run --rm --gpus all --cap-add SYS_ADMIN \
  -v $PWD/profiles:/profiles:ro \
  -e COSMIUM_PROFILE_NAME=win11_rtx3060_en-us \
  cosmium:gpu https://browserleaks.com/javascript
```

`--cap-add SYS_ADMIN` is required for Chromium's user-namespace sandbox. The
entrypoint deliberately does **not** pass `--no-sandbox`: a disabled sandbox is
itself a detectable signal on sites that probe sandbox status through the
Permissions API or process-hierarchy heuristics. Adding the capability is the
right trade.

## Entrypoint environment

`docker/entrypoint.sh` translates environment variables into Chromium flags,
then execs the browser. Anything you pass to `docker run` is appended after the
defaults.

| Variable | Purpose |
| --- | --- |
| `COSMIUM_PROFILE` | absolute path to a profile JSON |
| `COSMIUM_PROFILE_NAME` | name resolved as `/profiles/<name>.json` |
| `COSMIUM_CDP_PORT` | adds `--remote-debugging-port` |
| `COSMIUM_PROXY` | adds `--proxy-server` |
| `TZ` | overrides the timezone; otherwise read from the profile |

If the resolved profile path does not exist, the entrypoint exits rather than
starting an unspoofed browser.

The entrypoint also reads `locale.timezone` and `locale.languages[0]` out of the
profile with `jq` and exports `TZ`, `LANG`, and `LC_ALL` — matching what
`profile_to_env()` does natively, so the C++ patch and the process environment
agree.

Resolution is logged to **stderr** only, keeping stdout clean for
`--remote-debugging-pipe` usage.

## Default flags

The entrypoint always passes:

```
--headless=new
--disable-blink-features=AutomationControlled
--disable-features=Translate,BackForwardCache,InterestFeedContentSuggestions
--disable-background-timer-throttling
--disable-backgrounding-occluded-windows
--disable-renderer-backgrounding
--no-default-browser-check
--no-first-run
--remote-debugging-pipe
```

The three backgrounding flags matter for scraping: a headless browser whose
tabs get throttled produces different timing behavior than a foreground one,
and timing is measurable from JavaScript.

## Compose

```bash
# build environment — long-running, holds the source tree
docker compose -f docker/docker-compose.yml run --rm build \
  cargo run --release -p cosmium-cli -- build

# GPU runtime against a profile
PROFILE=win11_rtx3060_en-us \
docker compose -f docker/docker-compose.yml run --rm gpu https://example.com
```

The compose file parameterizes `CHROMIUM_TAG` (default `135.0.7049.84`) and
`PROFILE` (default `win11_rtx3060_en-us`), and mounts `profiles/` read-only.

Named volumes `cosmium-src`, `cosmium-out`, `cosmium-depot`, and `cosmium-cache`
persist the build tree — see [building Chromium](/operations/building/#protect-the-volumes).

## Driving it from an automation client

The runtime images expose an ordinary Chromium. Point any CDP client at it:

```bash
docker run --rm --gpus all --cap-add SYS_ADMIN -p 9222:9222 \
  -v $PWD/profiles:/profiles:ro \
  -e COSMIUM_PROFILE_NAME=win11_rtx3060_en-us \
  -e COSMIUM_CDP_PORT=9222 \
  cosmium:gpu
```

Then connect with puppeteer, playwright, or chromiumoxide as you would to any
remote Chrome. Alternatively run [`cosmium serve`](/guides/server/) inside the
container and use the HTTP API instead.
