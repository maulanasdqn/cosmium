---
title: HTTP server mode
description: Run Cosmium as a long-lived scraping service with an authenticated JSON API.
---

`cosmium serve` starts an axum server that exposes scraping and profile
management over HTTP. Use it instead of shelling out per page when you want a
single long-lived service, a shared profile store, or a language other than Rust
driving the scraper.

```bash
cosmium serve --port 3000
```

:::caution
`--api-key` defaults to `dev-key`. Change it before the port is reachable by
anything but your laptop.
:::

```bash
COSMIUM_API_KEY=$(openssl rand -hex 32) \
COSMIUM_BINARY=/opt/cosmium/chrome \
cosmium serve --port 3000
```

`serve` is not present in every published `cosmium-cli` release — if the
subcommand is missing, [build from source](/start/installation/#from-source).

## Flags

| Flag | Env | Default | Purpose |
| --- | --- | --- | --- |
| `--port` | `COSMIUM_PORT` | `3000` | listen port, bound on `0.0.0.0` |
| `--binary` | `COSMIUM_BINARY` | resolved from env | Chromium binary to launch |
| `--api-key` | `COSMIUM_API_KEY` | `dev-key` | value required in `x-api-key` |
| `--deepseek-api-key` | `DEEPSEEK_API_KEY` | unset | enables AI profile generation |
| `--max-browsers` | `COSMIUM_MAX_BROWSERS` | `4` | warm browsers kept at once |
| `--browser-max-pages` | `COSMIUM_BROWSER_MAX_PAGES` | `50` | scrapes before a browser is recycled |
| `--browser-max-age-secs` | `COSMIUM_BROWSER_MAX_AGE_SECS` | `1800` | lifetime before a browser is recycled |
| `--browser-idle-secs` | `COSMIUM_BROWSER_IDLE_SECS` | `300` | idle time before a browser is closed |

## Authentication

Every route under `/api/v1` requires an `x-api-key` header matching the
configured key; a mismatch is a bare `401`. `/api/health` and
`/api/auth/verify` sit outside that middleware — `verify` checks the same header
itself and returns `200` or `401`, which is what a login screen calls.

CORS is permissive, so a browser dashboard on another origin can talk to it
directly. That also means the API key is the *only* thing protecting the
service. Do not expose it to the public internet without a reverse proxy in
front.

## Scraping over HTTP

```bash
curl -X POST http://localhost:3000/api/v1/scrape \
  -H "x-api-key: $COSMIUM_API_KEY" \
  -H 'content-type: application/json' \
  -d '{
    "url": "https://example.com",
    "profile": "win11_rtx3060_en-us",
    "extract": ["h1", ".price"],
    "screenshot": true,
    "wait_ms": 2000,
    "proxies": ["http://user:pass@host:8080"],
    "proxy_rotation": "round_robin",
    "retries": 2
  }'
```

The request body mirrors the CLI flags — same block detection, same retry
semantics, same proxy pool. Screenshots come back base64-encoded in the
response rather than written to disk, and `include_html` controls whether the
full HTML is inlined (it is omitted by default, since it dominates the payload).

Full field list in the [HTTP API reference](/reference/http-api/).

## Browser pool

The server keeps Chromium running between requests instead of launching it for
every scrape. Each profile and proxy pair gets one warm browser, and every
request opens a fresh tab in it, so only the first request for a pair pays the
launch cost. Concurrent requests for the same pair share the browser as
separate tabs.

Each browser stores its state in a data directory under the cache dir. Direct
scrapes use the profile's directory, and every proxy gets a directory of its
own (named by a hash of the proxy URL), so cookies earned through one exit IP
are never replayed through another.

A browser is closed and relaunched on the next request when any of these hold:

- it has served `--browser-max-pages` scrapes or is older than
  `--browser-max-age-secs`
- it crashed, stopped answering, or a scrape on it failed
- the profile file or the request's `headful` or `geo_sync` changed since it
  launched

Browsers idle for `--browser-idle-secs` are closed in the background. When a
new pair needs a browser and `--max-browsers` are already open, the one idle
longest is closed first. If every browser is busy the cap is exceeded rather
than making the request wait.

Retirement waits for in-flight scrapes on that browser to finish. On `SIGINT` or
`SIGTERM` the server stops accepting requests, lets running ones complete, and
closes every pooled browser before it exits.

Retries back off between attempts (1s, 2s, 4s, then 8s, plus up to 0.5s of
jitter) and cover failed attempts as well as blocked ones. A missing binary or
an unreadable profile fails immediately without retrying.

## Profile management

```bash
# list
curl http://localhost:3000/api/v1/profiles -H "x-api-key: $KEY"

# fetch one
curl http://localhost:3000/api/v1/profiles/win11_rtx3060_en-us -H "x-api-key: $KEY"

# generate from a persona (requires --deepseek-api-key)
curl -X POST http://localhost:3000/api/v1/profiles/generate \
  -H "x-api-key: $KEY" -H 'content-type: application/json' \
  -d '{"persona":"MacBook Pro M2, Amsterdam, nl-NL","name":"mac_m2_nl"}'

# save one
curl -X POST http://localhost:3000/api/v1/profiles/save \
  -H "x-api-key: $KEY" -H 'content-type: application/json' \
  -d '{"name":"mac_m2_nl","profile":{ /* ... */ }}'
```

Generation returns the profile **and its diagnostics** without saving, so you
can inspect before committing — a `generate` then `save` pair, not one call.

Note that server-side generation uses DeepSeek, configured with
`--deepseek-api-key`, while the CLI's `profile generate` uses OpenRouter with
`OPENROUTER_API_KEY`. They are separate settings; the endpoint returns `503` if
no key was supplied at startup.

Saved names are sanitized to alphanumerics, `-`, and `_` before being written as
`<name>.json` into the profiles directory.

## The bundled dashboard

`GET /` serves a small built-in page. The fuller React dashboard lives in
`apps/ui` and talks to this same API — build it and serve it however you like,
pointing it at the server's origin with the API key.

## Running it in Docker

The runtime images already contain a browser binary. Set the port and key and
run the same command inside the container:

```bash
docker run --rm -p 3000:3000 \
  -v $PWD/profiles:/profiles:ro \
  -e COSMIUM_PROFILES_DIR=/profiles \
  -e COSMIUM_BINARY=/opt/cosmium/chrome \
  -e COSMIUM_API_KEY=$KEY \
  cosmium:gpu cosmium serve --port 3000
```

Each scrape launches and shuts down a browser, so concurrency is bounded by
memory rather than by anything in the server. Size the container accordingly and
put a queue in front if you need throughput. See [Docker](/operations/docker/).
