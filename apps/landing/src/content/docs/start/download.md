---
title: Download
description: Download a prebuilt patched Chromium binary for Linux x86_64 instead of building it yourself.
---

Building the patched Chromium takes hours and around 100 GB of disk. Every
release ships a prebuilt binary, so you can skip all of that.

## Linux x86_64

| File | Contents |
| --- | --- |
| [`cosmium-browser-linux-x86_64.tar.gz`](https://github.com/maulanasdqn/cosmium/releases/latest/download/cosmium-browser-linux-x86_64.tar.gz) | patched `chrome`, ICU data, locales, SwiftShader, and a `cosmium.json` build manifest |
| [`cosmium-browser-linux-x86_64.tar.gz.sha256`](https://github.com/maulanasdqn/cosmium/releases/latest/download/cosmium-browser-linux-x86_64.tar.gz.sha256) | SHA-256 checksum of the archive |

Older versions and release notes are on the
[releases page](https://github.com/maulanasdqn/cosmium/releases).

```bash
curl -fLO https://github.com/maulanasdqn/cosmium/releases/latest/download/cosmium-browser-linux-x86_64.tar.gz
curl -fLO https://github.com/maulanasdqn/cosmium/releases/latest/download/cosmium-browser-linux-x86_64.tar.gz.sha256
sha256sum --check cosmium-browser-linux-x86_64.tar.gz.sha256

sudo mkdir -p /opt/cosmium
sudo tar -xzf cosmium-browser-linux-x86_64.tar.gz -C /opt/cosmium --strip-components=1

export COSMIUM_BINARY=/opt/cosmium/chrome
/opt/cosmium/chrome --version
```

The archive is about 215 MB compressed and 660 MB unpacked. Each release on
GitHub carries the browser next to the CLI binaries, so the `latest` links above
always point at the newest build.

## What is in the build

`cosmium.json` inside the archive records the Chromium version, the build
time, and exactly which patches from the [patch series](/reference/patches/)
are compiled in. Check it after extracting:

```bash
jq . /opt/cosmium/cosmium.json
```

## Runtime requirements

- A glibc-based Linux distribution on x86_64. The binary is built against
  Chromium's Debian sysroot and runs on current Debian, Ubuntu, Fedora, Arch,
  and similar.
- The usual Chromium shared libraries (NSS, GTK, ALSA, and so on). In a slim
  container, the runtime image in `docker/` lists the exact packages.
- The setuid `chrome_sandbox` helper is not shipped. Chromium uses the
  user-namespace sandbox instead, which needs unprivileged user namespaces
  enabled. Inside containers that disallow them, pass `--no-sandbox`.

## Then

Install the CLI and run the probe suite against the downloaded binary:

```bash
cargo install cosmium-cli
cosmium test fingerprint --profile win11_rtx3060_en-us
```

See [installation](/start/installation/) for the CLI and profiles, and
[building](/operations/building/) if you want to build the binary yourself.
