---
title: CLI reference
description: Every cosmium subcommand, flag, and default.
---

```
cosmium <COMMAND>

Commands:
  profile   Inspect, validate, and author fingerprint profiles
  run       Launch the patched browser with a profile
  scrape    Scrape a page over CDP
  serve     Run the HTTP API server
  build     Build the patched Chromium
  test      Verify spoofing
```

Profile arguments accept either a bare name — resolved against
`COSMIUM_PROFILES_DIR` — or a path. `cosmium scrape page` is the exception: it
takes a path.

## `cosmium profile`

### `validate`

```
cosmium profile validate <TARGET> [--strict]
```

Runs schema and coherence checks. Errors block; warnings are advisory unless
`--strict` promotes them to failures.

### `show`

```
cosmium profile show <TARGET>
```

Prints the parsed profile as JSON.

### `list`

```
cosmium profile list
```

Enumerates profiles in `COSMIUM_PROFILES_DIR`.

### `generate`

```
cosmium profile generate --persona <TEXT> --name <SLUG> [--output PATH] [--save]
```

Requires `OPENROUTER_API_KEY`. `--save` writes to the profiles directory;
`--output` writes to a specific path.

### `repair`

```
cosmium profile repair <TARGET> [--output PATH]
```

Requires `OPENROUTER_API_KEY`. Sends the profile plus its diagnostics to the
model and re-validates the result.

### `mutate`

```
cosmium profile mutate <TARGET> [--count N] [--hint TEXT] [--output-dir DIR] [--save]
```

Requires `OPENROUTER_API_KEY`. `--count` defaults to `5`.

## `cosmium run`

```
cosmium run --profile <TARGET> [--binary PATH] [--flag <CHROME-FLAG>]... [URL]...
```

| Flag | Env | Default |
| --- | --- | --- |
| `--profile` | — | required |
| `--binary` | `COSMIUM_BINARY` | `$COSMIUM_ROOT/out/cosmium/chrome` |
| `--flag` | — | repeatable, passed through verbatim |

Trailing arguments are opened as URLs. This launches an interactive browser with
no CDP attachment — for automation, add
`--flag --remote-debugging-port=9222` and connect your own client.

## `cosmium scrape page`

```
cosmium scrape page --profile <PATH> [OPTIONS] <URL>
```

| Flag | Default | Purpose |
| --- | --- | --- |
| `--profile` | required | path to the profile JSON |
| `--binary` | `COSMIUM_BINARY` | Chromium binary |
| `--wait-ms` | `0` | flat delay after load |
| `--wait-for-api` | — | wait for a response URL containing this substring |
| `--screenshot` | off | capture a JPEG |
| `--extract` | — | repeatable bare CSS selector |
| `--script` | — | JavaScript to evaluate; result keyed as `script` |
| `--headful` | off | show a window; otherwise `--headless=new` |
| `--format` | `json` | `json` or `html` |
| `--output-dir` | — | write `page.html`, `screenshot.jpg`, `cookies.json` |
| `--retries` | `0` | extra attempts when blocked; total = `1 + N` |
| `--proxy` | — | single proxy URL, ignored when a pool exists |
| `--proxy-file` | — | file of proxy URLs, one per line, `#` comments |
| `--proxy-list` | — | comma-separated proxy URLs |
| `--proxy-rotation` | `round-robin` | `round-robin` or `random` |
| `--proxy-cooldown` | `60` | seconds a failed proxy sits out |

`--extract` takes a bare selector, and the extracted text is keyed by that
selector string. Attributes, custom names, and limits require a
[workflow](/guides/workflows/).

## `cosmium serve`

```
cosmium serve [--port N] [--binary PATH] [--api-key KEY] [--deepseek-api-key KEY]
```

| Flag | Env | Default |
| --- | --- | --- |
| `--port` | `COSMIUM_PORT` | `3000` |
| `--binary` | `COSMIUM_BINARY` | resolved from env |
| `--api-key` | `COSMIUM_API_KEY` | `dev-key` |
| `--deepseek-api-key` | `DEEPSEEK_API_KEY` | unset |

See [HTTP server mode](/guides/server/).

## `cosmium build`

```
cosmium build [--from PHASE] [--only PHASE] [--jobs N] [--install-build-deps] [--tag VERSION]
```

| Flag | Purpose |
| --- | --- |
| `--from` | run this phase and every later one |
| `--only` | run exactly one phase; conflicts with `--from` |
| `--jobs` | parallelism passed to the build |
| `--install-build-deps` | run Chromium's dependency installer first |
| `--tag` | override the tag in `VERSION` |

With neither `--from` nor `--only`, all phases run. See
[building Chromium](/operations/building/).

## `cosmium test`

### `fingerprint`

```
cosmium test fingerprint --profile <TARGET> [--binary PATH]
```

Runs the probe suite and prints a pass/fail table. **Exits non-zero if any probe
fails.**

### `stealth`

```
cosmium test stealth --profile <TARGET> [--binary PATH] [--bot-check-url URL] [--headful] [--json]
```

Drives CreepJS, Pixelscan, and BrowserLeaks, plus `--bot-check-url` if given.
See [testing stealth](/guides/testing/).

## Global environment

Logging is controlled by `COSMIUM_LOG`, which takes a `tracing` filter and
defaults to `info,cosmium=debug,engine=debug`:

```bash
COSMIUM_LOG=debug cosmium scrape page --profile ./p.json https://example.com
```

Full list on the [environment](/reference/environment/) page.
