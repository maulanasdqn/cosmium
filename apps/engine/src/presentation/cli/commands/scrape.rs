use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use crate::domain::scraping::request::ProxyConfig;
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

    let workflow = build_workflow(&args.extract, &args.script);
    let max_attempts = 1 + args.retries;
    let mut result = None;

    for attempt in 1..=max_attempts {
        if attempt > 1 {
            tracing::info!(attempt, max_attempts, "retrying scrape");
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }

        let session = Arc::new(crate::infrastructure::runtime::CdpSessionRuntime::new());
        let uc = ScrapePage::new(state.profile_repo.clone(), session);

        let input = ScrapePageInput {
            profile: args.profile.clone(),
            binary: args.binary.clone().unwrap_or_else(|| state.binary.clone()),
            url: args.url.clone(),
            wait_ms: args.wait_ms,
            screenshot: args.screenshot,
            workflow: workflow.clone(),
            proxy: single_proxy.clone(),
            proxy_pool: pool.clone(),
            headful: args.headful,
            wait_for_api: args.wait_for_api.clone(),
        };

        let r = uc.execute(input).await?;

        if let Some(ref url) = r.proxy_used {
            tracing::info!(proxy = %url, blocked = r.blocked, "attempt finished");
        }

        if !r.blocked || attempt == max_attempts {
            result = Some(r);
            break;
        }

        tracing::warn!("page blocked, rotating proxy");
        result = Some(r);
    }

    let result = result.unwrap();
    print_result(&args.format, &result, &args.output_dir)?;
    save_artifacts(&args.output_dir, &result)?;

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
