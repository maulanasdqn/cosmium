---
title: Quickstart
description: From a validated profile to a scraped page and a stealth report.
---

This walks the full loop: pick a profile, confirm it is coherent, launch the
browser, scrape a page, and verify the spoofing actually landed.

## 1. Validate a profile

Start here even before you have a browser binary — an incoherent profile
produces a *worse* fingerprint than no spoofing at all.

```bash
cosmium profile list
cosmium profile validate win11_rtx3060_en-us
```

Diagnostics come back with a severity. **Warnings are advisory; errors block.**
Add `--strict` to make warnings fail too:

```bash
cosmium profile validate win11_rtx3060_en-us --strict
```

To see the parsed document rather than the raw file:

```bash
cosmium profile show win11_rtx3060_en-us
```

## 2. Launch the browser

`cosmium run` maps the profile to switches and starts the patched binary. Any
URLs you pass are opened as tabs.

```bash
export COSMIUM_BINARY=/opt/cosmium/chrome

cosmium run \
  --profile win11_rtx3060_en-us \
  https://browserleaks.com/javascript
```

Extra Chromium flags pass through with repeated `--flag`:

```bash
cosmium run --profile win11_rtx3060_en-us \
  --flag --remote-debugging-port=9222 \
  --flag --proxy-server=http://user:pass@host:8080
```

That second example is the escape hatch for driving Cosmium from puppeteer,
playwright, or chromiumoxide — it is an ordinary Chromium binary, so point your
existing client at the debugging port.

## 3. Scrape a page

`cosmium scrape page` attaches over CDP, runs a workflow, and prints a result
document.

```bash
cosmium scrape page \
  --profile profiles/win11_rtx3060_en-us.json \
  --wait-ms 3000 \
  --extract "h1" \
  --extract ".product-price" \
  --screenshot \
  --output-dir ./out \
  https://example.com
```

Each `--extract` is a **bare CSS selector**; its text content comes back in the
result under a key equal to the selector itself. `--output-dir` writes
`page.html`, `screenshot.jpg`, and `cookies.json` into that directory.

For anything richer — attributes, per-selector names, result limits, clicks,
scrolls — you need a workflow. See [workflows](/guides/workflows/).

Note the profile argument here is a **path**, while `cosmium run` and
`cosmium profile` accept a bare profile name resolved against
`COSMIUM_PROFILES_DIR`. Both accept paths.

## 4. Add a proxy

A single proxy:

```bash
cosmium scrape page --profile profiles/win11_rtx3060_en-us.json \
  --proxy http://user:pass@host:8080 \
  https://example.com
```

A rotating pool with health tracking and automatic retry on a block:

```bash
cosmium scrape page --profile profiles/win11_rtx3060_en-us.json \
  --proxy-file proxies.txt \
  --proxy-rotation round-robin \
  --proxy-cooldown 60 \
  --retries 3 \
  https://example.com
```

When a page comes back flagged as blocked, Cosmium marks that proxy failed,
puts it in cooldown, and retries on the next healthy one. See
[proxy rotation](/guides/proxies/).

## 5. Verify the spoofing

Two different checks, and you want both.

**Probe suite** — asks the browser, from inside a page, what it reports for each
spoofed surface and compares against the profile:

```bash
cosmium test fingerprint --profile win11_rtx3060_en-us
```

**Live detection sites** — drives real fingerprinting pages (CreepJS,
Pixelscan, BrowserLeaks) and scores the verdicts:

```bash
cosmium test stealth --profile win11_rtx3060_en-us --json
```

Add your own target with `--bot-check-url`. Details in
[testing stealth](/guides/testing/).

## 6. Run it as a service

For a long-lived scraping backend, run the HTTP server instead of shelling out
per page:

```bash
COSMIUM_API_KEY=$(openssl rand -hex 32) \
cosmium serve --port 3000
```

```bash
curl -X POST http://localhost:3000/api/v1/scrape \
  -H "x-api-key: $COSMIUM_API_KEY" \
  -H 'content-type: application/json' \
  -d '{"url":"https://example.com","profile":"win11_rtx3060_en-us","extract":["h1"]}'
```

The default API key is `dev-key` — change it before exposing the port. See the
[HTTP API reference](/reference/http-api/).

## Where to go next

- [Fingerprint profiles](/guides/profiles/) — the profile model and its rules
- [Workflows](/guides/workflows/) — clicks, scrolls, infinite scroll, pagination
- [CLI reference](/reference/cli/) — every flag
