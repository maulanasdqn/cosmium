use std::path::Path as FsPath;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::application::use_cases::mutate_profile::{MutateProfile, MutateProfileInput};
use crate::application::use_cases::repair_profile::{RepairProfile, RepairProfileInput};
use crate::domain::llm::LlmClient;
use crate::domain::profile::{Diagnostic, Profile, Severity, validation};

use super::AppState;
use super::dto::diagnostics_dto;
use super::dto_platform::{MutateRequest, MutateResponse, ProfileWithDiagnostics};
use super::error::{ApiError, ApiResult, api_error, internal, profile_load_failed};

const MAX_VARIANTS: usize = 5;

pub fn check_count(count: usize) -> Result<usize, String> {
    if (1..=MAX_VARIANTS).contains(&count) {
        Ok(count)
    } else {
        Err(format!("count must be between 1 and {MAX_VARIANTS}"))
    }
}

pub(super) fn require_llm(state: &AppState) -> Result<Arc<dyn LlmClient>, ApiError> {
    state
        .llm
        .clone()
        .ok_or_else(|| api_error(StatusCode::SERVICE_UNAVAILABLE, "LLM not configured"))
}

async fn load(state: &AppState, name: &str) -> Result<Profile, ApiError> {
    state
        .profile_repo
        .load(FsPath::new(name))
        .await
        .map_err(|e| profile_load_failed(name, &e))
}

fn with_diagnostics(
    profile: &Profile,
    diags: &[Diagnostic],
) -> Result<ProfileWithDiagnostics, ApiError> {
    Ok(ProfileWithDiagnostics {
        profile: serde_json::to_value(profile).map_err(internal)?,
        diagnostics: diagnostics_dto(diags),
    })
}

pub async fn repair(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<ProfileWithDiagnostics> {
    let llm = require_llm(&state)?;
    let profile = load(&state, &name).await?;
    let diagnostics = validation::validate(&profile);
    if diagnostics.iter().all(|d| d.severity != Severity::Error) {
        return Ok(Json(with_diagnostics(&profile, &diagnostics)?));
    }
    let out = RepairProfile::new(llm, state.llm_model.clone())
        .execute(RepairProfileInput {
            profile,
            diagnostics,
        })
        .await
        .map_err(internal)?;
    Ok(Json(with_diagnostics(&out.profile, &out.diagnostics)?))
}

pub async fn mutate(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<MutateRequest>,
) -> ApiResult<MutateResponse> {
    let count = check_count(req.count).map_err(|e| api_error(StatusCode::BAD_REQUEST, e))?;
    let llm = require_llm(&state)?;
    let reference = load(&state, &name).await?;
    let out = MutateProfile::new(llm, state.llm_model.clone())
        .execute(MutateProfileInput {
            reference,
            count,
            hint: req.hint,
        })
        .await
        .map_err(internal)?;
    let variants = out
        .variants
        .iter()
        .map(|v| with_diagnostics(&v.profile, &v.diagnostics))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Json(MutateResponse { variants }))
}

#[cfg(test)]
mod tests {
    use super::check_count;

    #[test]
    fn mutate_count_is_bounded() {
        assert!(check_count(0).is_err());
        assert_eq!(check_count(1), Ok(1));
        assert_eq!(check_count(5), Ok(5));
        assert!(check_count(6).is_err());
    }
}
