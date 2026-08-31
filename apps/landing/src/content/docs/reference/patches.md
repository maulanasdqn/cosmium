---
title: Patch series
description: All 28 Chromium patches, the file each touches, and the detection vector it closes.
---

Patches live in `patches/` and apply in the order listed in `patches/series`,
against the Chromium tag pinned in `VERSION` — currently **135.0.7049.84**.

Order matters. Later patches build on switch plumbing introduced by earlier
ones, so applying a subset is not generally safe.

## The series

| # | Patch | Primary file | Vector |
| --- | --- | --- | --- |
| 0001 | strip-navigator-webdriver | `blink/core/frame/navigator.cc` | `navigator.webdriver` always false |
| 0002 | spoof-webgl-vendor-renderer | `blink/modules/webgl/webgl_rendering_context_base.cc` | `UNMASKED_VENDOR_WEBGL` / `UNMASKED_RENDERER_WEBGL` — kills the SwiftShader tell |
| 0003 | spoof-client-hints-platform | `blink/core/frame/navigator_ua_data.cc` | `navigator.userAgentData.platform` |
| 0004 | spoof-navigator-platform | `blink/core/frame/navigator_id.cc` | `navigator.platform` |
| 0005 | spoof-navigator-languages | `blink/core/frame/navigator_language.cc` | `navigator.languages` |
| 0006 | spoof-hardware-concurrency | `blink/core/frame/navigator_concurrent_hardware.cc` | `navigator.hardwareConcurrency` — the cgroup leak |
| 0007 | spoof-device-memory | `blink/core/frame/navigator_device_memory.cc` | `navigator.deviceMemory` |
| 0008 | spoof-screen-color-depth | `blink/core/frame/screen.cc` | `screen.colorDepth` / `pixelDepth` |
| 0009 | spoof-max-touch-points | `blink/core/events/navigator_events.cc` | `navigator.maxTouchPoints` |
| 0010 | spoof-audio-context-latency | `blink/modules/webaudio/audio_context.cc` | `AudioContext.baseLatency` / `outputLatency` |
| 0011 | spoof-sec-ch-ua-branding | `components/embedder_support/user_agent_utils.cc` | HTTP-level `Sec-CH-UA` brand list |
| 0012 | fix-pdf-plugin-branding | `chrome/common/chrome_content_client.cc` | PDF plugin name mismatch |
| 0013 | disable-automation-controlled | `content/child/runtime_features.cc` | the `AutomationControlled` blink feature |
| 0014 | suppress-detection-infobars | `chrome/browser/ui/startup/infobar_utils.cc` | the "controlled by automated software" bar |
| 0015 | default-webrtc-ip-handling-public-only | `chrome/browser/ui/browser_ui_prefs.cc` | Docker bridge IPs in ICE candidates |
| 0016 | cosmium-timezone-override | `blink/core/timezone/timezone_controller.cc` | UTC timezone |
| 0017 | strip-headless-ua-prefix | `components/embedder_support/user_agent_utils.cc` | `HeadlessChrome/` in the UA |
| 0018 | neuter-cdp-detection-signals | `chrome/browser/devtools/protocol/emulation_handler.cc` and chromedriver | CDP attachment tells |
| 0019 | canvas-fingerprint-noise-injection | `blink/platform/graphics/cosmium_canvas_noise.h` | pixel-identical canvas hashes |
| 0020 | inject-audio-fingerprint-noise | `blink/modules/webaudio/audio_buffer.cc` | pixel-identical audio hashes |
| 0021 | spoof-chrome-version-in-ua-headers | `components/embedder_support/user_agent_utils.cc` | present a newer Chrome than the build |
| 0022 | spoof-battery-status-api | `blink/modules/battery/battery_manager.cc` | missing Battery API |
| 0023 | forward-cosmium-switches-to-renderer | `content/browser/renderer_host/render_process_host_impl.cc` | **switches reaching Blink at all** |
| 0024 | apply-platform-override-above-ua-reduction | `blink/core/execution_context/navigator_base.cc` | UA reduction overwriting the override |
| 0025 | spoof-ua-platform-at-source-not-accessor | `blink/core/frame/navigator_ua_data.cc` | override applied too late to be consistent |
| 0026 | allow-unsafe-buffers-in-noise-patches | noise patch headers | build fix for the noise patches |
| 0027 | keep-timezone-override-across-monitor-push | `blink/core/timezone/timezone_controller.cc` | override lost on monitor-config push |
| 0028 | spoof-screen-dimensions | `blink/core/frame/screen.cc` | `screen.width`/`height` stuck at 800×600 |

## Patch 0023 is load-bearing

Most spoofed APIs are implemented in Blink, which runs in the **renderer**
process. A `--cosmium-*` switch on the browser process command line does not
automatically appear on the renderer's, so
`base::CommandLine::ForCurrentProcess()->HasSwitch(...)` returns false there and
every override quietly returns the real value — while the browser process
accepts the flag without complaint.

Patch 0023 adds the switch names to `kSwitchNames` in
`PropagateBrowserCommandLineToRenderer`. If a build applies every patch except
this one, `cosmium test fingerprint` fails broadly with no obvious cause. It is
the first thing to check when spoofing appears to do nothing.

## Conventions

From `patches/README.md`:

- **One patch, one detection vector.** No bundling — it keeps rebases, selective
  disabling, and bisection tractable.
- **Read from the profile, never hardcode.** Patches make Chromium *consult* the
  profile; no values are baked into the binary.
- **Keep diffs surgical.** The smaller the diff, the cheaper the next Chromium
  tag.
- **Match real Chrome on a miss.** When a profile field is absent, fall back to
  Chromium's default rather than an obviously-spoofed sentinel.

Every patch header documents the target files, what changed, which profile
fields it reads, and a test page that exercises it.

## Authoring a new patch

```bash
./scripts/reset.sh                 # start clean
./scripts/03-apply-patches.sh      # apply the existing series
$EDITOR src/...                    # edit Chromium directly
./scripts/04-build.sh              # verify it compiles
./scripts/test-fingerprint.sh      # verify it fixes the vector
./scripts/make-patch.sh 0029-my-patch
```

Then reset and confirm it reapplies from scratch. Add the filename to `series`.

## Deferred work

`patches/design/` holds design docs for patches that are specified but not yet
authored — each names the target files in the Chromium tree and the vector it
would close. See `patches/design/README.md` for the backlog.

## Rebasing onto a new Chromium tag

```bash
# bump VERSION, then
./scripts/reset.sh
./scripts/02-checkout.sh
./scripts/03-apply-patches.sh
```

Conflicts surface at apply time. Because each patch is one vector against a
named file, a conflict tells you exactly which surface Chromium moved. See
[building Chromium](/operations/building/).
