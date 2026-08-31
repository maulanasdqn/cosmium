use std::sync::Arc;
use std::time::Instant;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput, ScrapePageOutput};
use crate::domain::scraping::proxy_pool::{ProxyPoolConfig, RotationStrategy};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::runtime::CdpSessionRuntime;
use crate::infrastructure::scraping::ProxyPool;

use super::AppState;
use super::dto::{ErrorResponse, ScrapeRequest, ScrapeResponse};

pub async fn scrape(
    State(state): State<AppState>,
    Json(req): Json<ScrapeRequest>,
) -> Result<Json<ScrapeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let started = Instant::now();
    let workflow = build_workflow(&req);
    let pool = build_pool(&req);
    let single_proxy = if pool.is_none() {
        req.proxy.clone().map(|url| ProxyConfig { url })
    } else {
        None
    };
    let url = req.url.clone();
    let include_html = req.include_html;
    let max_attempts = 1 + req.retries;

    let mut result = None;
    for attempt in 1..=max_attempts {
        if attempt > 1 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }

        let session = Arc::new(CdpSessionRuntime::new());
        let uc = ScrapePage::new(state.profile_repo.clone(), session);

        let input = ScrapePageInput {
            profile: std::path::PathBuf::from(&req.profile),
            binary: state.binary.clone(),
            url: req.url.clone(),
            wait_ms: req.wait_ms,
            screenshot: req.screenshot,
            workflow: workflow.clone(),
            proxy: single_proxy.clone(),
            proxy_pool: pool.clone(),
            headful: req.headful,
            wait_for_api: req.wait_for_api.clone(),
        };

        let r = uc.execute(input).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;

        if !r.blocked || attempt == max_attempts {
            result = Some((r, attempt));
            break;
        }
        result = Some((r, attempt));
    }

    let (r, attempts) = result.unwrap();
    Ok(Json(build_response(
        url,
        include_html,
        &r,
        &started,
        attempts,
    )))
}

fn build_pool(req: &ScrapeRequest) -> Option<Arc<ProxyPool>> {
    if req.proxies.is_empty() {
        return None;
    }

    let proxies: Vec<ProxyConfig> = req
        .proxies
        .iter()
        .map(|u| ProxyConfig { url: u.clone() })
        .collect();

    let strategy = match req.proxy_rotation.as_deref() {
        Some("random") => RotationStrategy::Random,
        _ => RotationStrategy::RoundRobin,
    };

    let config = ProxyPoolConfig {
        strategy,
        cooldown_secs: 60,
        max_failures: 3,
    };

    Some(Arc::new(ProxyPool::new(proxies, config)))
}

fn build_workflow(req: &ScrapeRequest) -> Vec<WorkflowStep> {
    let mut steps = Vec::new();
    for sel in &req.extract {
        steps.push(WorkflowStep::Extract {
            name: sel.clone(),
            selector: sel.clone(),
            attribute: None,
            limit: 0,
        });
    }
    if let Some(code) = &req.script {
        steps.push(WorkflowStep::Script {
            name: "script".into(),
            code: code.clone(),
            timeout_seconds: 30,
        });
    }
    steps
}

fn build_response(
    url: String,
    include_html: bool,
    result: &ScrapePageOutput,
    started: &Instant,
    attempts: u32,
) -> ScrapeResponse {
    let mut extracted = serde_json::Map::new();
    for (k, v) in &result.page.script_results {
        let parsed: serde_json::Value =
            serde_json::from_str(v).unwrap_or(serde_json::Value::String(v.clone()));
        extracted.insert(k.clone(), parsed);
    }

    let screenshot_base64 = if result.page.screenshot.is_empty() {
        None
    } else {
        use base64::Engine as _;
        Some(base64::engine::general_purpose::STANDARD.encode(&result.page.screenshot))
    };

    let html = if include_html {
        Some(String::from_utf8_lossy(&result.page.html).into_owned())
    } else {
        None
    };

    ScrapeResponse {
        url,
        final_url: result.page.final_url.clone(),
        http_status: result.page.http_status,
        html_length: result.page.html.len(),
        html,
        user_agent: result.page.user_agent.clone(),
        cookies_count: result.page.cookies.len(),
        blocked: result.blocked,
        extracted: serde_json::Value::Object(extracted),
        screenshot_base64,
        proxy_used: result.proxy_used.clone(),
        elapsed_ms: started.elapsed().as_millis() as u64,
        attempts,
    }
}
