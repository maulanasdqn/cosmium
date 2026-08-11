use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::presentation::cli::state::CliState;

use super::scrape_output::build_json_output;

#[derive(Debug, Subcommand)]
pub enum ScrapeCmd {
    Page(ScrapePageArgs),
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
}

pub async fn execute(cmd: ScrapeCmd, state: &CliState) -> Result<()> {
    match cmd {
        ScrapeCmd::Page(args) => execute_page(args, state).await,
    }
}

async fn execute_page(args: ScrapePageArgs, state: &CliState) -> Result<()> {
    let session = std::sync::Arc::new(crate::infrastructure::runtime::CdpSessionRuntime::new());
    let uc = ScrapePage::new(state.profile_repo.clone(), session);

    let mut workflow = Vec::new();
    for sel in &args.extract {
        workflow.push(WorkflowStep::Extract {
            name: sel.clone(),
            selector: sel.clone(),
            attribute: None,
            limit: 0,
        });
    }
    if let Some(code) = &args.script {
        workflow.push(WorkflowStep::Script {
            name: "script".into(),
            code: code.clone(),
            timeout_seconds: 30,
        });
    }

    let proxy = args.proxy.map(|url| ProxyConfig { url });

    let max_attempts = 1 + args.retries;
    let mut result = None;
    for attempt in 1..=max_attempts {
        if attempt > 1 {
            tracing::info!(attempt, max_attempts, "retrying scrape");
        }
        let input = ScrapePageInput {
            profile: args.profile.clone(),
            binary: args.binary.clone().unwrap_or_else(|| state.binary.clone()),
            url: args.url.clone(),
            wait_ms: args.wait_ms,
            screenshot: args.screenshot,
            workflow: workflow.clone(),
            proxy: proxy.clone(),
            headful: args.headful,
        };
        let r = uc.execute(input).await?;
        if !r.blocked || attempt == max_attempts {
            result = Some(r);
            break;
        }
        tracing::warn!("page blocked, retrying with new proxy IP");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        result = Some(r);
    }
    let result = result.unwrap();

    match args.format.as_str() {
        "html" => {
            let html = String::from_utf8_lossy(&result.page.html);
            println!("{html}");
        }
        _ => {
            let output = build_json_output(&result, &args.output_dir)?;
            println!("{output}");
        }
    }

    if result.blocked {
        eprintln!("warning: page appears to be blocked");
    }

    if let Some(dir) = &args.output_dir {
        std::fs::create_dir_all(dir)?;
        let html_path = dir.join("page.html");
        std::fs::write(&html_path, &result.page.html)?;
        eprintln!("saved html to {}", html_path.display());

        if !result.page.screenshot.is_empty() {
            let ss_path = dir.join("screenshot.jpg");
            std::fs::write(&ss_path, &result.page.screenshot)?;
            eprintln!("saved screenshot to {}", ss_path.display());
        }

        if !result.page.cookies.is_empty() {
            let cookies_path = dir.join("cookies.json");
            let cookies_json = serde_json::to_string_pretty(&result.page.cookies)?;
            std::fs::write(&cookies_path, cookies_json)?;
            eprintln!("saved cookies to {}", cookies_path.display());
        }
    }

    Ok(())
}
