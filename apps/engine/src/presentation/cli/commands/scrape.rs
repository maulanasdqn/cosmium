use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use crate::domain::scraping::request::ProxyConfig;
use crate::infrastructure::scraping::{BrowserPool, PoolLimits};
use crate::presentation::cli::state::CliState;

use super::scrape_output::{build_proxy_pool, build_workflow, print_result, save_artifacts};

#[derive(Debug, Subcommand)]
pub enum ScrapeCmd {
    Page(ScrapePageArgs),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RotationArg {
    RoundRobin,
    Random,
}

#[derive(Debug, Args)]
pub struct ScrapePageArgs {
    #[arg(long)]
    pub profile: PathBuf,

    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<PathBuf>,

    pub url: String,

    #[arg(long, default_value_t = 0)]
    pub wait_ms: u32,

    #[arg(long)]
    pub screenshot: bool,

    #[arg(long)]
    pub proxy: Option<String>,

    #[arg(long)]
    pub proxy_file: Option<PathBuf>,

    #[arg(long, value_delimiter = ',')]
    pub proxy_list: Vec<String>,

    #[arg(long, default_value = "round-robin")]
    pub proxy_rotation: RotationArg,

    #[arg(long, default_value_t = 60)]
    pub proxy_cooldown: u64,

    #[arg(long)]
    pub output_dir: Option<PathBuf>,

    #[arg(long)]
    pub extract: Vec<String>,

    #[arg(long)]
    pub script: Option<String>,

    #[arg(long)]
    pub headful: bool,

    #[arg(long, default_value = "json")]
    pub format: String,

    #[arg(long, default_value_t = 0)]
    pub retries: u32,

    #[arg(long)]
    pub wait_for_api: Option<String>,
    #[arg(long)]
    pub geo_sync: bool,
}

pub async fn execute(cmd: ScrapeCmd, state: &CliState) -> Result<()> {
    match cmd {
        ScrapeCmd::Page(args) => execute_page(args, state).await,
    }
}

async fn execute_page(args: ScrapePageArgs, state: &CliState) -> Result<()> {
    let pool = build_proxy_pool(&args)?;
    let single_proxy = if pool.is_none() {
        args.proxy.clone().map(|url| ProxyConfig { url })
    } else {
        None
    };

    if let Some(ref p) = pool {
        tracing::info!(count = p.len(), "proxy pool initialized");
    }

    let browsers = BrowserPool::start(PoolLimits {
        max_browsers: 1,
        ..PoolLimits::default()
    });
    let uc = ScrapePage::new(Arc::clone(&state.profile_repo), Arc::clone(&browsers));
    let input = ScrapePageInput {
        profile: args.profile.clone(),
        binary: args.binary.clone().unwrap_or_else(|| state.binary.clone()),
        url: args.url.clone(),
        wait_ms: args.wait_ms,
        screenshot: args.screenshot,
        workflow: build_workflow(&args.extract, args.script.as_deref()),
        proxy: single_proxy,
        proxy_pool: pool.clone(),
        headful: args.headful,
        wait_for_api: args.wait_for_api.clone(),
        geo_sync: args.geo_sync,
    };
    let outcome = uc.execute_with_retries(&input, args.retries).await;
    browsers.shutdown().await;
    let (result, attempts) = outcome?;
    tracing::info!(attempts, blocked = result.blocked, "scrape finished");

    print_result(&args.format, &result, args.output_dir.as_deref())?;
    save_artifacts(args.output_dir.as_deref(), &result)?;

    if let Some(ref p) = pool {
        for s in &p.stats() {
            tracing::debug!(
                proxy = %s.url, uses = s.total_uses,
                failures = s.total_failures, available = s.available,
                "proxy stats"
            );
        }
    }

    if result.blocked {
        tracing::warn!("page appears to be blocked");
    }

    Ok(())
}
