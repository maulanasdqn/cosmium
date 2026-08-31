---
title: Environment
description: Every environment variable Cosmium reads, its default, and what it controls.
---

A `.env` file at `COSMIUM_ROOT` is loaded automatically on startup, so local
development usually needs nothing exported.

## Paths

| Variable | Default | Purpose |
| --- | --- | --- |
| `COSMIUM_ROOT` | current working directory | root used to resolve every other path |
| `COSMIUM_PROFILES_DIR` | `$COSMIUM_ROOT/profiles` | where profiles are read and saved |
| `COSMIUM_PATCHES_DIR` | `$COSMIUM_ROOT/patches` | patch series location |
| `COSMIUM_BUILD_OUT` | `$COSMIUM_ROOT/out/cosmium` | gn output directory |
| `COSMIUM_BINARY` | `$COSMIUM_BUILD_OUT/chrome` | binary the browser commands launch |

`COSMIUM_ROOT` defaults to wherever you invoke the CLI. If you installed with
`cargo install` and run from your home directory, the profiles directory will
not exist — set `COSMIUM_PROFILES_DIR` explicitly.

## Logging

| Variable | Default |
| --- | --- |
| `COSMIUM_LOG` | `info,cosmium=debug,engine=debug` |

Takes a `tracing` env-filter string. Useful settings:

```bash
COSMIUM_LOG=debug                    # everything, including proxy pool stats
COSMIUM_LOG=warn                     # quiet
COSMIUM_LOG=info,engine=trace        # deep on the engine only
```

Proxy pool per-entry statistics are only printed at `debug`.

## Server

| Variable | Default | Flag |
| --- | --- | --- |
| `COSMIUM_PORT` | `3000` | `--port` |
| `COSMIUM_API_KEY` | `dev-key` | `--api-key` |
| `DEEPSEEK_API_KEY` | unset | `--deepseek-api-key` |

Change `COSMIUM_API_KEY` before exposing the port. See
[HTTP server mode](/guides/server/).

## LLM authoring

Read by `profile generate`, `repair`, and `mutate`. No other subcommand needs
them.

| Variable | Default | Purpose |
| --- | --- | --- |
| `OPENROUTER_API_KEY` | unset | required for the three LLM subcommands |
| `OPENROUTER_BASE_URL` | `https://openrouter.ai/api/v1` | any OpenAI-compatible endpoint |
| `OPENROUTER_MODEL` | `anthropic/claude-sonnet-4.6` | any model with JSON mode |
| `OPENROUTER_REFERER` | unset | sent as `HTTP-Referer` |
| `OPENROUTER_TITLE` | unset | sent as `X-Title` |

Note the asymmetry: the **CLI** uses OpenRouter, while the **server's**
generation endpoint uses DeepSeek via `DEEPSEEK_API_KEY`. They are configured
separately.

## Set by Cosmium, not read by it

When launching the browser, `profile_to_env()` sets three variables on the child
process from the profile:

| Variable | Value |
| --- | --- |
| `TZ` | `locale.timezone` |
| `LANG` | POSIX form of `locale.languages[0]`, e.g. `en_US.UTF-8` |
| `LC_ALL` | same as `LANG` |

These are derived from the profile — setting them in your own shell has no
effect on the launched browser.

## An example `.env`

```bash
COSMIUM_ROOT=/srv/cosmium
COSMIUM_PROFILES_DIR=/srv/cosmium/profiles
COSMIUM_BINARY=/opt/cosmium/chrome
COSMIUM_LOG=info,engine=debug

COSMIUM_PORT=3000
COSMIUM_API_KEY=replace-me

OPENROUTER_API_KEY=sk-or-v1-...
OPENROUTER_MODEL=anthropic/claude-sonnet-4.6
```
