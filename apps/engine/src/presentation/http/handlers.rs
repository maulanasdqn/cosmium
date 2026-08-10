use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::application::use_cases::generate_profile::{GenerateProfile, GenerateProfileInput};

use super::AppState;
use super::dto::{
    DiagnosticDto, ErrorResponse, GenerateProfileRequest, GenerateProfileResponse,
    ProfileListResponse, SaveProfileRequest, SaveProfileResponse,
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

    Ok(Json(SaveProfileResponse { saved: name }))
}
