---
title: Embedding the engine
description: Use cosmium-engine as a library — map profiles to flags, or drive a full scrape from Rust.
---

`cosmium-engine` is the library behind the CLI. Its ports-and-adapters layout
means you can take as little as the flag mapper or as much as the whole scrape
use case.

```toml
[dependencies]
cosmium-engine = "0.2"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

Or from git:

```toml
cosmium-engine = { git = "https://github.com/maulanasdqn/cosmium", package = "cosmium-engine" }
```

The crate root exposes four modules — `domain`, `application`,
`infrastructure`, `presentation` — plus `cli` re-exported from `presentation`.

## Just the flag mapper

The smallest useful integration. If you already have an automation stack and
only want Cosmium's profile-to-switch mapping:

```rust
use cosmium_engine::domain::runtime::{profile_to_env, profile_to_flags};
use cosmium_engine::infrastructure::profile::FsJsonProfileRepository;
use cosmium_engine::domain::profile::ProfileRepository;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let repo = FsJsonProfileRepository::new("./profiles");
    let profile = repo.load(Path::new("win11_rtx3060_en-us")).await?;

    let flags: Vec<String> = profile_to_flags(&profile);
    let env: Vec<(String, String)> = profile_to_env(&profile);

    // hand `flags` and `env` to chromiumoxide, fantoccini, or a raw Command
    Ok(())
}
```

`profile_to_flags` is pure — no I/O, no async — so it is trivial to test against
and cheap to call.

## A full scrape

```rust
use std::sync::Arc;
use cosmium_engine::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use cosmium_engine::infrastructure::profile::FsJsonProfileRepository;
use cosmium_engine::infrastructure::runtime::CdpSessionRuntime;
use cosmium_engine::domain::scraping::workflow::WorkflowStep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let repo = Arc::new(FsJsonProfileRepository::new("./profiles"));
    let session = Arc::new(CdpSessionRuntime::new());
    let uc = ScrapePage::new(repo, session);

    let workflow = vec![
        WorkflowStep::Delay { duration_ms: 1500 },
        WorkflowStep::Extract {
            name: "title".into(),
            selector: "h1".into(),
            attribute: None,
            limit: 0,
        },
    ];

    let out = uc.execute(ScrapePageInput {
        profile: "profiles/win11_rtx3060_en-us.json".into(),
        binary: "/opt/cosmium/chrome".into(),
        url: "https://example.com".into(),
        wait_ms: 0,
        screenshot: false,
        workflow,
        proxy: None,
        proxy_pool: None,
        headful: false,
        wait_for_api: None,
    }).await?;

    println!("blocked={} status={}", out.blocked, out.page.http_status);
    Ok(())
}
```

This is the path to the [workflow steps](/guides/workflows/) the CLI does not
expose — `click`, `input`, `scroll`, `follow_urls`.

A `CdpSessionRuntime` is single-use: it launches a browser and shuts it down at
the end of `execute`. Construct a fresh one per scrape, as the CLI does on each
retry attempt.

## With a proxy pool

```rust
use std::sync::Arc;
use cosmium_engine::domain::scraping::proxy_pool::{
    parse_proxy_list, ProxyPoolConfig, RotationStrategy,
};
use cosmium_engine::infrastructure::scraping::ProxyPool;

let proxies = parse_proxy_list(&std::fs::read_to_string("proxies.txt")?);
let pool = Arc::new(ProxyPool::new(proxies, ProxyPoolConfig {
    strategy: RotationStrategy::RoundRobin,
    cooldown_secs: 60,
    max_failures: 3,
}));

// then pass `proxy_pool: Some(pool.clone())` in ScrapePageInput
```

The pool is `Send + Sync` and internally locked, so one instance can be shared
across concurrent scrapes. It marks proxies healthy or failed automatically
based on the block verdict — see [proxy rotation](/guides/proxies/).

## Swapping an adapter

Every infrastructure type implements a domain port, so you can substitute your
own. `ProfileRepository` is the easy example:

```rust
use async_trait::async_trait;
use std::path::Path;
use cosmium_engine::domain::profile::{Profile, ProfileRepository};
use cosmium_engine::domain::profile::error::ProfileResult;

struct PostgresProfiles { /* ... */ }

#[async_trait]
impl ProfileRepository for PostgresProfiles {
    async fn load(&self, path: &Path) -> ProfileResult<Profile> { todo!() }
    async fn list(&self) -> ProfileResult<Vec<String>> { todo!() }
}
```

Pass your implementation wherever `Arc<dyn ProfileRepository>` is expected and
every use case keeps working. The same pattern applies to `BrowserRuntime`,
`BrowserSession`, and `LlmClient`.

## Validating profiles in your own pipeline

```rust
use cosmium_engine::application::use_cases::validate_profile::ValidateProfile;
```

Worth running on anything user-supplied. An incoherent profile is a worse
fingerprint than no spoofing, so treat validation as a gate rather than a
diagnostic — the reasoning is on the
[profiles guide](/guides/profiles/#coherence-rules).
