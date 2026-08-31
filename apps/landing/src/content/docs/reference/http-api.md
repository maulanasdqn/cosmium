---
title: HTTP API
description: Endpoints, request and response shapes for cosmium serve.
---

Base URL is the server's origin; all API routes are nested under `/api`. Start
the server with [`cosmium serve`](/guides/server/).

## Authentication

Routes under `/api/v1` require an `x-api-key` header matching the configured
key. A mismatch returns `401` with an empty body.

```
x-api-key: <COSMIUM_API_KEY>
```

CORS is permissive on every route.

## Routes

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `GET` | `/` | no | bundled dashboard page |
| `GET` | `/api/health` | no | liveness, always `200` |
| `POST` | `/api/auth/verify` | header checked in-handler | `200` or `401` |
| `GET` | `/api/v1/profiles` | yes | list profile names |
| `GET` | `/api/v1/profiles/{name}` | yes | fetch one profile |
| `POST` | `/api/v1/profiles/generate` | yes | LLM generation |
| `POST` | `/api/v1/profiles/save` | yes | write a profile to disk |
| `POST` | `/api/v1/scrape` | yes | scrape a page |

## `POST /api/v1/scrape`

### Request

| Field | Type | Default | Notes |
| --- | --- | --- | --- |
| `url` | string | required | page to fetch |
| `profile` | string | required | profile name |
| `screenshot` | bool | `false` | returns base64 JPEG |
| `headful` | bool | `false` | show a window |
| `wait_ms` | number | `0` | flat delay after load |
| `extract` | string[] | `[]` | bare CSS selectors |
| `script` | string | null | JavaScript, result keyed `script` |
| `proxy` | string | null | single proxy, ignored if `proxies` is set |
| `proxies` | string[] | `[]` | proxy pool |
| `proxy_rotation` | string | `round_robin` | `round_robin` or `random` |
| `include_html` | bool | `false` | inline the full HTML |
| `wait_for_api` | string | null | wait for a matching response URL |
| `retries` | number | `0` | extra attempts when blocked |

```json
{
  "url": "https://example.com",
  "profile": "win11_rtx3060_en-us",
  "extract": ["h1", ".price"],
  "screenshot": true,
  "wait_ms": 2000,
  "retries": 2
}
```

### Response

| Field | Type | Notes |
| --- | --- | --- |
| `url` | string | requested URL |
| `final_url` | string | after redirects |
| `http_status` | number | |
| `html_length` | number | always present |
| `html` | string? | only when `include_html` is true |
| `user_agent` | string | as the page saw it |
| `cookies_count` | number | |
| `blocked` | bool | see [block detection](/guides/scraping/#block-detection) |
| `extracted` | object | keyed by selector |
| `screenshot_base64` | string? | only when `screenshot` is true |
| `proxy_used` | string? | which proxy served the attempt |
| `elapsed_ms` | number | |
| `attempts` | number | how many tries it took |

```json
{
  "url": "https://example.com",
  "final_url": "https://example.com/",
  "http_status": 200,
  "html_length": 51234,
  "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) …",
  "cookies_count": 3,
  "blocked": false,
  "extracted": { "h1": ["Example Domain"] },
  "elapsed_ms": 4820,
  "attempts": 1
}
```

`html` is omitted rather than sent empty when `include_html` is false — the same
applies to `screenshot_base64` and `proxy_used`.

## `GET /api/v1/profiles`

```json
{ "profiles": ["win11_rtx3060_en-us", "macos_m2_en-us"] }
```

## `GET /api/v1/profiles/{name}`

Returns the parsed profile document, or `404` with
`{"error":"..."}` if it does not exist.

## `POST /api/v1/profiles/generate`

Requires the server to have been started with `--deepseek-api-key`; otherwise
returns `503`.

```json
{ "persona": "MacBook Pro M2, Amsterdam, nl-NL", "name": "mac_m2_nl" }
```

Response carries the profile **and** its validation diagnostics. Nothing is
written to disk — follow up with `save` if you want to keep it.

```json
{
  "profile": { "name": "mac_m2_nl", "identity": { } },
  "diagnostics": [
    { "severity": "warning", "code": "…", "message": "…" }
  ]
}
```

## `POST /api/v1/profiles/save`

```json
{ "name": "mac_m2_nl", "profile": { } }
```

The name is sanitized to alphanumerics, `-`, and `_`, then written as
`<name>.json` into the profiles directory. An empty name after sanitizing is a
`400`.

```json
{ "saved": "mac_m2_nl" }
```

## Errors

Failures return the appropriate status with a uniform body:

```json
{ "error": "loading profiles/missing.json: No such file or directory" }
```

| Status | Cause |
| --- | --- |
| `400` | malformed body or invalid profile name |
| `401` | missing or wrong `x-api-key` |
| `404` | profile not found |
| `500` | scrape or filesystem failure |
| `503` | LLM endpoint called without a key configured |
