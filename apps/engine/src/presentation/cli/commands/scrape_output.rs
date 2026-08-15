use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;

use crate::application::use_cases::scrape_page::ScrapePageOutput;
use crate::domain::scraping::proxy_pool::{ProxyPoolConfig, RotationStrategy, parse_proxy_list};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::scraping::ProxyPool;

use super::scrape::ScrapePageArgs;
use super::scrape::RotationArg;

pub fn build_json_output(result: &ScrapePageOutput, output_dir: &Option<PathBuf>) -> Result<String> {
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

    if let Some(ref proxy) = result.proxy_used {
        obj.insert(
            "proxy_used".into(),
            serde_json::Value::String(proxy.clone()),
        );
    }

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

pub fn print_result(format: &str, result: &ScrapePageOutput, output_dir: &Option<PathBuf>) -> Result<()> {
    match format {
        "html" => {
            let html = String::from_utf8_lossy(&result.page.html);
            println!("{html}");
        }
        _ => {
            let output = build_json_output(result, output_dir)?;
            println!("{output}");
        }
    }
    Ok(())
}

pub fn save_artifacts(output_dir: &Option<PathBuf>, result: &ScrapePageOutput) -> Result<()> {
    let Some(dir) = output_dir else {
        return Ok(());
    };
    std::fs::create_dir_all(dir)?;

    let html_path = dir.join("page.html");
    std::fs::write(&html_path, &result.page.html)?;
    tracing::info!(path = %html_path.display(), "saved html");

    if !result.page.screenshot.is_empty() {
        let ss_path = dir.join("screenshot.jpg");
        std::fs::write(&ss_path, &result.page.screenshot)?;
        tracing::info!(path = %ss_path.display(), "saved screenshot");
    }

    if !result.page.cookies.is_empty() {
        let cookies_path = dir.join("cookies.json");
        let cookies_json = serde_json::to_string_pretty(&result.page.cookies)?;
        std::fs::write(&cookies_path, cookies_json)?;
        tracing::info!(path = %cookies_path.display(), "saved cookies");
    }

    Ok(())
}

pub fn build_proxy_pool(args: &ScrapePageArgs) -> Result<Option<Arc<ProxyPool>>> {
    let mut proxies = Vec::new();

    if let Some(ref path) = args.proxy_file {
        let content = std::fs::read_to_string(path)?;
        proxies.extend(parse_proxy_list(&content));
    }

    for url in &args.proxy_list {
        proxies.push(ProxyConfig { url: url.clone() });
    }

    if proxies.is_empty() {
        return Ok(None);
    }

    let strategy = match args.proxy_rotation {
        RotationArg::RoundRobin => RotationStrategy::RoundRobin,
        RotationArg::Random => RotationStrategy::Random,
    };

    let config = ProxyPoolConfig {
        strategy,
        cooldown_secs: args.proxy_cooldown,
        max_failures: 3,
    };

    Ok(Some(Arc::new(ProxyPool::new(proxies, config))))
}

pub fn build_workflow(extract: &[String], script: &Option<String>) -> Vec<WorkflowStep> {
    let mut workflow = Vec::new();
    for sel in extract {
        workflow.push(WorkflowStep::Extract {
            name: sel.clone(),
            selector: sel.clone(),
            attribute: None,
            limit: 0,
        });
    }
    if let Some(code) = script {
        workflow.push(WorkflowStep::Script {
            name: "script".into(),
            code: code.clone(),
            timeout_seconds: 30,
        });
    }
    workflow
}
