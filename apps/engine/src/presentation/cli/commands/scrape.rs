use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::presentation::cli::state::CliState;

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

    let result = uc
        .execute(ScrapePageInput {
            profile: args.profile,
            binary: args.binary.unwrap_or_else(|| state.binary.clone()),
            url: args.url,
            wait_ms: args.wait_ms,
            screenshot: args.screenshot,
            workflow,
            proxy,
            headful: args.headful,
        })
        .await?;

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

fn build_json_output(
    result: &crate::application::use_cases::scrape_page::ScrapePageOutput,
    output_dir: &Option<PathBuf>,
) -> Result<String> {
    let mut obj = serde_json::Map::new();
    obj.insert(
        "url".into(),
        serde_json::Value::String(result.page.final_url.clone()),
    );
    obj.insert(
        "status".into(),
        serde_json::Value::Number(result.page.http_status.into()),
    );
    obj.insert(
        "html_length".into(),
        serde_json::Value::Number(result.page.html.len().into()),
    );
    obj.insert("blocked".into(), serde_json::Value::Bool(result.blocked));
    obj.insert(
        "user_agent".into(),
        serde_json::Value::String(result.page.user_agent.clone()),
    );
    obj.insert(
        "cookies_count".into(),
        serde_json::Value::Number(result.page.cookies.len().into()),
    );

    if !result.page.script_results.is_empty() {
        let mut extracted = serde_json::Map::new();
        for (k, v) in &result.page.script_results {
            let parsed: serde_json::Value =
                serde_json::from_str(v).unwrap_or(serde_json::Value::String(v.clone()));
            extracted.insert(k.clone(), parsed);
        }
        obj.insert("extracted".into(), serde_json::Value::Object(extracted));
    }

    if !result.page.screenshot.is_empty() {
        if let Some(dir) = output_dir {
            obj.insert(
                "screenshot_path".into(),
                serde_json::Value::String(dir.join("screenshot.jpg").display().to_string()),
            );
        } else {
            obj.insert(
                "screenshot_bytes".into(),
                serde_json::Value::Number(result.page.screenshot.len().into()),
            );
        }
    }

    Ok(serde_json::to_string_pretty(&serde_json::Value::Object(
        obj,
    ))?)
}
