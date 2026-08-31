---
title: Profile schema
description: Every field in a Cosmium fingerprint profile, its type, and its allowed values.
---

The authoritative definition is `profiles/schema.json` in the repo. This page
mirrors it. Twelve sections are required: `name`, `identity`, `locale`,
`hardware`, `gpu`, `screen`, `audio`, `media_devices`, `voices`, `fonts`,
`webrtc`, `canvas_noise`.

Unknown top-level keys are rejected.

## Top level

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `name` | string | yes | profile id, e.g. `win11_rtx3060_en-us` |
| `version` | integer | no | schema version, currently `1` |
| `strip_automation_tells` | boolean | no | opt-in; emits the matching switch |
| `chrome_version` | string | no | rewrites the UA's Chrome version |

## `identity`

Required: `user_agent`, `client_hints`, `navigator_platform`,
`navigator_app_version`.

| Field | Type | Notes |
| --- | --- | --- |
| `user_agent` | string | full UA string |
| `client_hints` | object | see below |
| `navigator_platform` | string | `Win32`, `MacIntel`, `Linux x86_64` |
| `navigator_app_version` | string | |
| `navigator_vendor` | string | defaults to `Google Inc.` |
| `navigator_product` | string | defaults to `Gecko` |
| `navigator_product_sub` | string | defaults to `20030107` |

### `identity.client_hints`

All fields required: `brands`, `platform`, `platform_version`, `architecture`,
`bitness`, `model`, `mobile`, `wow64`.

| Field | Type | Allowed |
| --- | --- | --- |
| `brands` | array of `{brand, version}` | |
| `platform` | string | `Windows`, `macOS`, `Linux` |
| `platform_version` | string | e.g. `15.0.0` |
| `architecture` | string | `x86`, `arm` |
| `bitness` | string | `32`, `64` |
| `model` | string | usually empty on desktop |
| `mobile` | boolean | |
| `wow64` | boolean | |

The Chrome entry in `brands` must carry the same major version as the UA string.

## `locale`

Required: `languages`, `accept_language`, `timezone`.

| Field | Type | Notes |
| --- | --- | --- |
| `languages` | string[] | `navigator.languages`, e.g. `["en-US","en"]` |
| `accept_language` | string | HTTP header form, e.g. `en-US,en;q=0.9` |
| `timezone` | string | IANA zone, e.g. `America/Chicago` |
| `currency` | string | optional |

`accept_language` is stored in header form because that is what it means on the
wire. The `--accept-lang` switch gets a stripped bare list — Chromium's parser
`CHECK`-fails on `;` or a space and aborts the browser at startup.

## `hardware`

Required: `hardware_concurrency`, `device_memory_gb`, `max_touch_points`.

| Field | Type | Allowed |
| --- | --- | --- |
| `hardware_concurrency` | integer | must be even |
| `device_memory_gb` | number | `0.25`, `0.5`, `1`, `2`, `4`, `8` |
| `max_touch_points` | integer | `0` on non-touch machines |
| `battery` | object | optional |

### `hardware.battery`

Optional. When present, emits the `--cosmium-battery-*` quartet.

| Field | Type | Notes |
| --- | --- | --- |
| `charging` | boolean | |
| `level` | number | `0.0`–`1.0` |
| `charging_time_seconds` | integer? | null becomes `Infinity` |
| `discharging_time_seconds` | integer? | null becomes `Infinity` |

Omit this section entirely for a desktop. A desktop reporting a battery is as
wrong as a laptop reporting none.

## `gpu`

Required: `vendor`, `renderer`, `vendor_id`, `device_id`.

| Field | Type | Notes |
| --- | --- | --- |
| `vendor` | string | e.g. `Google Inc. (NVIDIA)` |
| `renderer` | string | e.g. `ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0, D3D11)` |
| `vendor_id` | string | PCI vendor id |
| `device_id` | string | PCI device id |
| `webgl_version` | string | optional |
| `webgpu_adapter` | object | optional |

`renderer` must not contain `SwiftShader` or `0x0000C0DE`.

## `screen`

Required: `width`, `height`, `avail_width`, `avail_height`, `color_depth`,
`pixel_depth`, `device_pixel_ratio`.

| Field | Type | Allowed |
| --- | --- | --- |
| `width`, `height` | integer | the display |
| `avail_width`, `avail_height` | integer | minus OS chrome |
| `avail_left`, `avail_top` | integer | optional origin offset |
| `color_depth` | integer | `24`, `30` |
| `pixel_depth` | integer | `24`, `30` — must equal `color_depth` |
| `device_pixel_ratio` | number | `1`, `1.25`, `1.5`, `1.75`, `2`, `2.5`, `3` |

The avail box should be smaller than the display by a plausible amount — a
Windows taskbar is about 48 device-independent pixels, a macOS menu bar about
25.

## `audio`

Required: `sample_rate`, `base_latency`, `output_latency`.

| Field | Type | Allowed |
| --- | --- | --- |
| `sample_rate` | integer | `44100`, `48000` |
| `base_latency` | number | seconds |
| `output_latency` | number | seconds |
| `max_channel_count` | integer | optional |

## `media_devices`

An array. Must contain at least one `audioinput`, one `audiooutput`, and one
`videoinput` — an empty `enumerateDevices()` is the single strongest container
tell.

## `voices`

An array describing `speechSynthesis.getVoices()`. Serialized to JSON and passed
as `--cosmium-voices`.

## `fonts`

Required: `installed`, an array of font family names. Passed as a
comma-separated `--cosmium-fonts`. Match the list to the platform — a Windows
profile without `Segoe UI` is not a Windows machine.

## `webrtc`

Required: `ip_handling_policy`.

| Field | Type | Allowed |
| --- | --- | --- |
| `ip_handling_policy` | string | `default`, `default_public_interface_only`, `default_public_and_private_interfaces`, `disable_non_proxied_udp` |
| `stun_servers` | array | optional |

`default_public_interface_only` is what keeps Docker bridge IPs out of ICE
candidates.

## `canvas_noise`

Required: `seed` — exactly 32 hexadecimal characters. Drives deterministic
per-profile canvas and audio noise, so the same profile produces a stable
fingerprint across runs while different profiles diverge.

## `browser_state`

Optional.

| Field | Type | Notes |
| --- | --- | --- |
| `history_length` | integer | `window.history.length` |
| `download_count` | integer | |
| `extensions` | array | reported extension entries |

## Reference profiles

`profiles/win11_rtx3060_en-us.json` and `profiles/macos_m2_en-us.json` are the
best worked examples — realistic values for every field, and the fixtures the
test suite asserts against.
