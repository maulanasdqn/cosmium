use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};

use crate::application::use_cases::generate_profile::{GenerateProfile, GenerateProfileInput};

use super::AppState;
use super::dto::{
    ErrorResponse, GenerateProfileRequest, GenerateProfileResponse, ProfileListResponse,
    SaveProfileRequest, SaveProfileResponse, diagnostics_dto,
};
use super::dto_platform::HealthResponse;
use super::error::sanitize_name;

const INDEX_HTML: &str = include_str!("index.html");

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        binary_found: state.binary.is_file(),
        llm_configured: state.llm.is_some(),
    })
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

    let diagnostics = diagnostics_dto(&output.diagnostics);

    Ok(Json(GenerateProfileResponse {
        profile: profile_value,
        diagnostics,
    }))
}

pub async fn save_profile(
    State(state): State<AppState>,
    Json(req): Json<SaveProfileRequest>,
) -> Result<Json<SaveProfileResponse>, (StatusCode, Json<ErrorResponse>)> {
    let Some(name) = sanitize_name(&req.name) else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "invalid profile name".into(),
            }),
        ));
    };

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
