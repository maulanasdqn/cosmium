---
title: Chromium switches
description: Every flag profile_to_flags emits, the profile field behind it, and the process environment.
---

`profile_to_flags()` is the contract between a profile and the patched binary.
It is a pure function with no I/O, unit-tested against both reference profiles,
because a rename on either side would stop the spoofing silently rather than
failing loudly.

## Cosmium switches

Introduced by the [patch series](/reference/patches/). Each is emitted on every
run unless marked conditional.

| Switch | Profile field |
| --- | --- |
| `--cosmium-platform` | `identity.navigator_platform` |
| `--cosmium-ua-platform` | `identity.client_hints.platform` |
| `--cosmium-languages` | `locale.languages`, comma-joined |
| `--cosmium-hardware-concurrency` | `hardware.hardware_concurrency` |
| `--cosmium-device-memory` | `hardware.device_memory_gb` |
| `--cosmium-color-depth` | `screen.color_depth` |
| `--cosmium-max-touch-points` | `hardware.max_touch_points` |
| `--cosmium-webgl-vendor` | `gpu.vendor` |
| `--cosmium-webgl-renderer` | `gpu.renderer` |
| `--cosmium-audio-base-latency` | `audio.base_latency` |
| `--cosmium-audio-output-latency` | `audio.output_latency` |
| `--cosmium-audio-sample-rate` | `audio.sample_rate` |
| `--cosmium-audio-max-channels` | `audio.max_channel_count` |
| `--cosmium-canvas-seed` | `canvas_noise.seed` |
| `--cosmium-voices` | `voices`, JSON-encoded |
| `--cosmium-fonts` | `fonts.installed`, comma-joined |
| `--cosmium-media-devices` | `media_devices`, JSON-encoded |
| `--cosmium-screen-width` | `screen.width` |
| `--cosmium-screen-height` | `screen.height` |
| `--cosmium-avail-width` | `screen.avail_width` |
| `--cosmium-avail-height` | `screen.avail_height` |
| `--cosmium-avail-left` | `screen.avail_left` |
| `--cosmium-avail-top` | `screen.avail_top` |
| `--cosmium-timezone` | `locale.timezone` |

### Conditional

| Switch | Emitted when |
| --- | --- |
| `--cosmium-strip-automation-tells` | `strip_automation_tells` is true — always forced on by `scrape` |
| `--cosmium-chrome-version` | `chrome_version` is set |
| `--cosmium-battery-charging` | `hardware.battery` is present |
| `--cosmium-battery-level` | `hardware.battery` is present |
| `--cosmium-battery-charging-time` | `hardware.battery` is present; null becomes `Infinity` |
| `--cosmium-battery-discharging-time` | `hardware.battery` is present; null becomes `Infinity` |

:::caution[Screen dimensions need both]
`--window-size` sizes the window; `--cosmium-screen-*` sets what
`screen.width`/`height` report. Without the latter, the display stays at the
headless 800×600 and contradicts the `avail_*` values emitted beside it.
:::

## Standard Chromium switches

| Switch | Source |
| --- | --- |
| `--user-agent` | `identity.user_agent`, rewritten if `chrome_version` is set |
| `--lang` | `locale.languages[0]` |
| `--accept-lang` | `locale.accept_language`, q-values stripped |
| `--window-size` | `screen.width`,`screen.height` |
| `--force-device-scale-factor` | `screen.device_pixel_ratio` |
| `--force-webrtc-ip-handling-policy` | `webrtc.ip_handling_policy` |

### Why `--accept-lang` is stripped

The profile stores `accept_language` in HTTP header form (`en-US,en;q=0.9`),
but Chromium's switch parser trips a `CHECK` in `net/http/http_util.cc` on a
`;` or a space:

```
Check failed: std::string::npos == language.find_first_of("; ")
```

That aborts the browser during startup rather than failing softly. The mapper
reduces the value to a bare comma-separated list before it reaches the switch.

## Hygiene flags

Emitted unconditionally to quiet first-run behavior and background chatter:

```
--no-default-browser-check
--no-first-run
--no-pings
--disable-domain-reliability
--disable-component-update
--disable-search-engine-choice-screen
--password-store=basic
--use-mock-keychain
--disable-blink-features=AutomationControlled
--disable-infobars
```

## `--disable-features`

One switch carrying the full list:

```
Translate, InterestFeedContentSuggestions, PrivacySandboxAdsAPIs,
OptimizationHints, MediaRouter, DialMediaRouteProvider, AcceptCHFrame,
AutofillServerCommunication, CertificateTransparencyComponentUpdater,
GlobalMediaControls, ImprovedCookieControls, LazyFrameLoading,
PreloadMediaEngagementData, MediaEngagementBypassAutoplayPolicies
```

`AcceptCHFrame` is the notable one — leaving it enabled lets a server request
Client Hints over the HTTP/2 ALPS frame, a path separate from the JS-side
spoofing.

## Process environment

Flags are not the whole story. `profile_to_env()` also sets three variables on
the browser process:

| Variable | Value |
| --- | --- |
| `TZ` | `locale.timezone` |
| `LANG` | POSIX form of `locale.languages[0]`, e.g. `en_US.UTF-8` |
| `LC_ALL` | same as `LANG` |

`TZ` matters because parts of the stack read the system timezone rather than
Blink's override. Setting both is belt and braces, and patch 0027 exists because
the Blink-side override was being lost on a monitor-configuration push.

## Runtime additions

Beyond the mapper, `cosmium scrape` adds:

- `--headless=new` unless `--headful`
- `--cosmium-strip-automation-tells`, always
- `--proxy-server=…` pointing at the local forwarder when a proxy is configured
- a per-profile `--user-data-dir`, so cookies survive between runs

## Inspecting what your profile emits

```bash
COSMIUM_LOG=debug cosmium run --profile win11_rtx3060_en-us
```

The launch spec is logged before the process starts.
