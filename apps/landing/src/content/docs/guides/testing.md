---
title: Testing stealth
description: The two verification commands — a per-surface probe suite and live detection-site scoring.
---

Cosmium ships two verification commands that answer different questions. Run
both; passing one and failing the other is common and informative.

| Command | Question it answers |
| --- | --- |
| `cosmium test fingerprint` | Does the browser report what the profile says? |
| `cosmium test stealth` | Do real detection sites flag it? |

## `test fingerprint` — per-surface probes

Executes a JavaScript expression per spoofed surface inside a real page and
compares the result against the loaded profile. Because the expected value is
*derived from the profile*, the report tells you exactly which patches landed.

```bash
cosmium test fingerprint --profile win11_rtx3060_en-us
```

```text
PROBE                  RESULT   VALUE
---------------------- -------- ----------------------------------------
webdriver              PASS     false
webdriver_worker       PASS     false
platform               PASS     Win32
languages              PASS     ["en-US","en"]
webgl_renderer         PASS     ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 ...
no_swiftshader         PASS     ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 ...
timezone               FAIL  expected=America/Chicago  UTC

passed=30 failed=1
```

The command **exits non-zero if any probe fails**, so it drops straight into CI
or a post-build gate.

### What it probes

Roughly thirty checks, grouped:

- **Automation tells** — `webdriver` in the main realm and again inside a Web
  Worker, `has_focus`, `visibility`, `pdf_viewer`, `chrome_app`, `chrome_csi`
- **Identity** — `user_agent`, `platform`, `ua_data_platform`, `ua_data_arch`,
  `ua_data_bitness`, `ua_data_pv`
- **Locale** — `language`, `languages`, `timezone`, `intl_locale`, `intl_tz`
- **Hardware** — `hardware_concurrency`, `device_memory`, `max_touch_points`
- **GPU** — `webgl_vendor`, `webgl_renderer`, and `no_swiftshader`, which is a
  negated check: it fails if the renderer string contains `SwiftShader` at all
- **Screen** — `screen_width`, `screen_height`, `screen_avail_width`,
  `screen_avail_height`, `color_depth`, `device_pixel_ratio`
- **Audio** — `audio_sample_rate`, `audio_base_latency`,
  `audio_max_channel_count`

The `webdriver_worker` probe deserves attention: it spawns a `Worker` from a
blob and asks *it* for `navigator.webdriver`. JavaScript-injection stealth
tooling typically fails this one, which is a good illustration of why Cosmium
patches the C++ instead.

## `test stealth` — live detection sites

Drives real fingerprinting pages, extracts their verdict, and scores it.

```bash
cosmium test stealth --profile win11_rtx3060_en-us
cosmium test stealth --profile win11_rtx3060_en-us --json
cosmium test stealth --profile win11_rtx3060_en-us --headful
```

Built-in targets:

| Target | URL | Settle time |
| --- | --- | --- |
| `creepjs` | `abrahamjuliot.github.io/creepjs` | 25 s |
| `pixelscan` | `pixelscan.net` | 10 s |
| `browserleaks` | `browserleaks.com/javascript` | 8 s |

Add your own — typically the actual site you are trying to reach, or a vendor's
demo page:

```bash
cosmium test stealth --profile win11_rtx3060_en-us \
  --bot-check-url https://your-target.example/login
```

Each target reports one of three verdicts:

- **`✓` Pass** — the site's own scoring came back clean
- **`~` Warn** — inconclusive, most often because the page did not finish
  loading inside its settle window
- **`✗` Fail** — flagged

`--json` emits the structured results — target, verdict, detail, duration — for
pipelines. `--headful` opens a visible window so you can read the site's report
yourself, which is usually what you want the first time a target starts failing.

## Reading the two together

The combinations are diagnostic:

- **Fingerprint passes, stealth fails.** The spoofing works, but something above
  the browser gives you away — IP reputation, request pacing, behavior, or a
  profile that is internally coherent yet implausible for the traffic pattern.
- **Fingerprint fails, stealth passes.** A patch is not landing. The target you
  tested simply is not checking that surface *yet*. Fix it anyway.
- **Both fail on a fresh build.** Check that patch 0023 applied — it forwards
  the `--cosmium-*` switches to the renderer process, and without it many
  overrides silently return real values while the browser accepts every flag
  without complaint.

## In CI

`test fingerprint` is the one to automate — it is deterministic, needs no
network beyond the browser, and exits non-zero on failure:

```bash
cosmium test fingerprint \
  --profile win11_rtx3060_en-us \
  --binary out/cosmium/chrome
```

`test stealth` depends on third-party sites that change their scoring without
notice, so it is better run on demand than in a gate.
