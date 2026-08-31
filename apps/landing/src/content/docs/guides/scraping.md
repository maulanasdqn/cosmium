---
title: Scraping a page
description: How cosmium scrape page works — launch, extraction, block detection, retries, and artifacts.
---

`cosmium scrape page` is the batteries-included path: it launches the patched
browser, attaches over CDP, navigates, runs a workflow, decides whether the page
was blocked, and prints a result document.

```bash
cosmium scrape page \
  --profile profiles/win11_rtx3060_en-us.json \
  --wait-ms 3000 \
  --extract "h1" \
  --screenshot \
  --output-dir ./out \
  https://example.com
```

## What happens on each run

1. The profile is loaded and mapped to switches via `profile_to_flags()`.
2. `--headless=new` is appended **unless** you pass `--headful`.
3. `--cosmium-strip-automation-tells` is force-enabled — scraping always opts in,
   even when the profile does not.
4. A per-profile user data directory is used, so cookies and local storage
   persist between runs of the same profile.
5. If a proxy is configured, a local forwarder starts (see below) and the
   browser is pointed at it.
6. The page is navigated, the workflow runs, and HTML, cookies, extracted
   values, and an optional screenshot are collected.
7. Block detection runs over the result.
8. The browser shuts down.

## Waiting

Three different knobs, for three different situations:

- **`--wait-ms`** — a flat delay after load. Blunt but reliable for pages that
  settle on a timer.
- **`--wait-for-api <substring>`** — wait until a network response whose URL
  contains the substring is captured. This is the one you want for SPAs that
  render from an XHR; it also captures the raw API response, which is often
  cleaner than scraping the DOM it produced.
- **workflow `delay` steps** — waits interleaved between actions. See
  [workflows](/guides/workflows/).

## Extraction

Each `--extract` is a bare CSS selector. The text content comes back keyed by
the selector string itself:

```bash
--extract "h1" --extract ".price" --extract "article p"
```

For attributes, named keys, or result limits, use a workflow instead — the CLI
flag deliberately stays simple.

Arbitrary JavaScript runs with `--script`:

```bash
cosmium scrape page --profile ./p.json \
  --script "JSON.stringify([...document.querySelectorAll('a')].map(a=>a.href))" \
  https://example.com
```

The script's return value appears in the result under the key `script`, parsed
as JSON when it parses and returned as a string when it does not. Scripts get a
30-second timeout.

## Block detection

Cosmium classifies the result rather than trusting the status code alone.
A page counts as blocked when:

- the status is **403, 429, or 503** *and* the body either looks like a
  challenge page or is too short to be real content; or
- the final URL landed on a wall — it contains `captcha`,
  `account-verification`, `negative_traffic`, `/challenge`, `verify/traffic`,
  `/verify/bot`, or `/blocked`; or
- the first 4 KB of the body contains a block marker: `captcha`,
  `are you a robot`, `access denied`, `cf-browser-verification`,
  `just a moment`, or `unusual traffic`.

Challenge-page recognition is broader still, covering Cloudflare
(`just a moment`, `checking your browser`, `attention required`), DataDome
(`captcha-delivery.com`), AWS WAF (`awswaf.com`), PerimeterX (`press & hold`),
and several localized variants.

The verdict shows up as `"blocked": true` in the JSON result and drives both
retries and proxy health.

## Retries

```bash
--retries 3
```

`--retries N` means up to `N + 1` attempts. A retry happens only when the
attempt came back blocked; a clean page returns immediately. There is a fixed
two-second pause between attempts, and each attempt takes a fresh proxy from
the pool if one is configured.

## Proxies

```bash
# single
--proxy http://user:pass@host:8080

# pool
--proxy-file proxies.txt
--proxy-list http://a:8080,http://b:8080
--proxy-rotation round-robin      # or: random
--proxy-cooldown 60
```

Authenticated proxies are handled by a local forwarder process rather than by
`--proxy-server` directly, which avoids Chromium's credential prompt. If the
forwarder cannot start, Cosmium falls back to a plain `--proxy-server` flag.
See [proxy rotation](/guides/proxies/) for the health model.

## Output

`--format json` (the default) prints a document with `url`, `status`,
`html_length`, `blocked`, `user_agent`, `cookies_count`, and — when present —
`extracted`, `proxy_used`, and either `screenshot_path` or `screenshot_bytes`.

`--format html` prints the raw page HTML to stdout instead, which pipes nicely
into other tools. Those are the only two formats.

`--output-dir DIR` writes artifacts to disk:

| File | Written when |
| --- | --- |
| `page.html` | always |
| `screenshot.jpg` | `--screenshot` was passed |
| `cookies.json` | the page set cookies |

## Headful debugging

When a site behaves differently under automation and you want to watch:

```bash
cosmium scrape page --profile ./p.json --headful https://example.com
```

Everything else stays identical, so what you see is what the headless run gets.
