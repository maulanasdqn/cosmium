use std::path::Path as FsPath;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::domain::profile::{Profile, validation};

use super::AppState;
use super::dto::diagnostics_dto;
use super::dto_platform::{
    DeletedResponse, DiagnosticsResponse, ProfileSummariesResponse, ValidateRequest,
};
use super::error::{
    ApiResult, api_error, internal, profile_error, profile_load_failed, sanitize_name,
};
use super::summary::{failed, summarize};

pub async fn summaries(State(state): State<AppState>) -> ApiResult<ProfileSummariesResponse> {
    let mut names = state.profile_repo.list().await.map_err(internal)?;
    names.sort();
    let mut profiles = Vec::with_capacity(names.len());
    for name in names {
        let summary = match state.profile_repo.load(FsPath::new(&name)).await {
            Ok(p) => summarize(&name, &p),
            Err(e) => failed(&name, profile_error(&name, &e).1),
        };
        profiles.push(summary);
    }
    Ok(Json(ProfileSummariesResponse { profiles }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(raw): Path<String>,
) -> ApiResult<DeletedResponse> {
    let name = sanitize_name(&raw)
        .ok_or_else(|| api_error(StatusCode::BAD_REQUEST, "invalid profile name"))?;
    let path = state.profiles_dir.join(format!("{name}.json"));
    if !tokio::fs::try_exists(&path).await.unwrap_or(false) {
        return Err(api_error(
            StatusCode::NOT_FOUND,
            format!("profile not found: {name}"),
        ));
    }
    tokio::fs::remove_file(&path).await.map_err(internal)?;
    Ok(Json(DeletedResponse { deleted: name }))
}

pub async fn validate_named(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<DiagnosticsResponse> {
    let profile = state
        .profile_repo
        .load(FsPath::new(&name))
        .await
        .map_err(|e| profile_load_failed(&name, &e))?;
    Ok(Json(DiagnosticsResponse {
        diagnostics: diagnostics_dto(&validation::validate(&profile)),
    }))
}

pub async fn validate_body(Json(req): Json<ValidateRequest>) -> ApiResult<DiagnosticsResponse> {
    let profile: Profile = serde_json::from_value(req.profile)
        .map_err(|e| api_error(StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(DiagnosticsResponse {
        diagnostics: diagnostics_dto(&validation::validate(&profile)),
    }))
}
