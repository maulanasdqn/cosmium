---
title: How it works
description: The path from a JSON profile to a launched browser, and how the Rust workspace is layered.
---

Cosmium has two halves that meet at the command line: a patched Chromium that
reads `--cosmium-*` switches, and a Rust engine that turns a profile into those
switches.

## The pipeline

```
profiles/win11_rtx3060_en-us.json
          │
          │  FsJsonProfileRepository::load
          ▼
     Profile (domain entity)
          │
          │  validation/  — cross-field coherence rules
          ▼
     Profile (validated)
          │
          │  profile_to_flags()
          ▼
  ["--user-agent=…", "--cosmium-platform=Win32", …]
          │
          │  TokioProcessRuntime / CdpSessionRuntime
          ▼
      patched chrome binary
```

`profile_to_flags()` is a pure function with no I/O, which is why it is
exhaustively unit-tested — the flag names are a contract with the C++ patches,
and a rename on either side would silently stop spoofing rather than fail
loudly.

## Two runtimes

The engine has two ways to start the browser, chosen by the subcommand:

- **`TokioProcessRuntime`** — used by [`cosmium run`](/reference/cli/#cosmium-run).
  Spawns the binary with the mapped flags and hands you an interactive browser.
  No CDP connection; you drive it yourself or just look at it.
- **`CdpSessionRuntime`** — used by [`cosmium scrape`](/guides/scraping/) and
  `cosmium test`. Launches the browser, attaches over the Chrome DevTools
  Protocol, runs a workflow, and collects HTML, cookies, extracted values, and
  screenshots.

## Workspace layout

The Rust workspace follows clean architecture — dependencies point only inward,
so the domain never imports from infrastructure:

```
apps/engine/src/
├── domain/                    entities, value objects, ports
│   ├── profile/               Profile + validation/ coherence rules
│   ├── runtime/               BrowserRuntime port, flags/ switch mapper
│   ├── scraping/              workflow, request, proxy_pool, validation
│   └── llm/                   LlmClient port
├── application/use_cases/     one file per use case
│   ├── validate_profile.rs    list_profiles.rs   run_browser.rs
│   ├── scrape_page.rs         test_fingerprint/  validate_stealth/
│   └── generate_profile.rs    repair_profile.rs  mutate_profile.rs
├── infrastructure/            adapters implementing the ports
│   ├── profile/               FsJsonProfileRepository
│   ├── runtime/               TokioProcessRuntime, CdpSessionRuntime
│   ├── scraping/              network capture, warmup, proxy_pool
│   └── llm/                   OpenRouterClient
└── presentation/
    ├── cli/                   clap parser + command handlers
    └── http/                  axum router, DTOs, handlers
```

Two house rules the CI enforces: **max 200 lines per file**, and zero clippy
warnings under `-D warnings`.

## Where the patches live

Each patch in `patches/` is a git-format patch against the Chromium tag pinned
in `VERSION` (currently `135.0.7049.84`). `patches/series` lists them in apply
order — the order matters, since later patches (0023–0028) build on switch
plumbing introduced by earlier ones.

Patch 0023 is worth calling out: `--cosmium-*` switches reach the browser
process by default, but the renderer process is where most of these APIs are
implemented. That patch forwards the switches across the process boundary, so
several earlier patches only take effect once it is applied.
