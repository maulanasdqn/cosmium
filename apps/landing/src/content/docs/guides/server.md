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
