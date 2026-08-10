use std::sync::Arc;
use std::time::Instant;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput, ScrapePageOutput};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::runtime::CdpSessionRuntime;

use super::AppState;
use super::dto::{ErrorResponse, ScrapeRequest, ScrapeResponse};

pub async fn scrape(
    State(state): State<AppState>,
    Json(req): Json<ScrapeRequest>,
) -> Result<Json<ScrapeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let started = Instant::now();
    let session = Arc::new(CdpSessionRuntime::new());
    let uc = ScrapePage::new(state.profile_repo.clone(), session);

    let workflow = build_workflow(&req);
    let proxy = req.proxy.map(|url| ProxyConfig { url });
    let url = req.url.clone();
    let include_html = req.include_html;

    let input = ScrapePageInput {
        profile: std::path::PathBuf::from(&req.profile),
        binary: state.binary.clone(),
        url: req.url,
        wait_ms: req.wait_ms,
        screenshot: req.screenshot,
        workflow,
        proxy,
        headful: req.headful,
    };

    let result = uc.execute(input).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    Ok(Json(build_response(url, include_html, &result, &started)))
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
        elapsed_ms: started.elapsed().as_millis() as u64,
    }
}
