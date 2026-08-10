use std::sync::Arc;
use std::time::Instant;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::application::use_cases::generate_profile::{GenerateProfile, GenerateProfileInput};
use crate::application::use_cases::scrape_page::{ScrapePage, ScrapePageInput};
use crate::domain::scraping::request::ProxyConfig;
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::runtime::CdpSessionRuntime;

use super::AppState;
use super::dto::{
    DiagnosticDto, ErrorResponse, GenerateProfileRequest, GenerateProfileResponse,
    ProfileListResponse, SaveProfileRequest, SaveProfileResponse, ScrapeRequest, ScrapeResponse,
};

const INDEX_HTML: &str = include_str!("index.html");

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn health() -> StatusCode {
    StatusCode::OK
}

pub async fn verify_token(
    State(state): State<AppState>,
    request: axum::http::Request<axum::body::Body>,
) -> StatusCode {
    let provided = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    if provided == state.api_key {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    }
}

pub async fn list_profiles(
    State(state): State<AppState>,
) -> Result<Json<ProfileListResponse>, (StatusCode, Json<ErrorResponse>)> {
    let profiles = state.profile_repo.list().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;
    Ok(Json(ProfileListResponse { profiles }))
}

pub async fn get_profile(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let profile = state
        .profile_repo
        .load(std::path::Path::new(&name))
        .await
        .map_err(|e| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: e.to_string(),
                }),
            )
        })?;
    Ok(Json(profile))
}

pub async fn generate_profile(
    State(state): State<AppState>,
    Json(req): Json<GenerateProfileRequest>,
) -> Result<Json<GenerateProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let llm = state.llm.ok_or_else(|| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ErrorResponse {
                error: "AI not configured — set DEEPSEEK_API_KEY".into(),
            }),
        )
    })?;

    let uc = GenerateProfile::new(llm, state.llm_model);
    let input = GenerateProfileInput {
        persona: req.persona,
        name: req.name,
    };

    let output = uc.execute(input).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    let profile_value = serde_json::to_value(&output.profile).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    let diagnostics = output
        .diagnostics
        .iter()
        .map(|d| DiagnosticDto {
            severity: format!("{:?}", d.severity).to_lowercase(),
            code: d.field.to_string(),
            message: d.message.clone(),
        })
        .collect();

    Ok(Json(GenerateProfileResponse {
        profile: profile_value,
        diagnostics,
    }))
}

pub async fn save_profile(
    State(state): State<AppState>,
    Json(req): Json<SaveProfileRequest>,
) -> Result<Json<SaveProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let name = req
        .name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();
    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "invalid profile name".into(),
            }),
        ));
    }

    let path = state.profiles_dir.join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&req.profile).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    tokio::fs::write(&path, json).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
    })?;

    Ok(Json(SaveProfileResponse {
        saved: name,
    }))
}

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
    result: &crate::application::use_cases::scrape_page::ScrapePageOutput,
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
