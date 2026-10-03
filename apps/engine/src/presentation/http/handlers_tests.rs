use std::path::{Path as FsPath, PathBuf};
use std::sync::Arc;

use axum::Json;
use axum::extract::State;

use crate::application::use_cases::test_fingerprint::{TestFingerprint, TestFingerprintInput};
use crate::application::use_cases::validate_stealth::targets::builtin_targets;
use crate::application::use_cases::validate_stealth::{ValidateStealth, ValidateStealthInput};
use crate::domain::scraping::validation::Verdict;
use crate::infrastructure::runtime::CdpSessionRuntime;

use super::AppState;
use super::dto_platform::{
    FingerprintRequest, FingerprintResponse, ProbeDto, StealthRequest, StealthResponse,
    StealthResultDto,
};
use super::error::{ApiResult, internal, profile_load_failed};

pub const fn verdict_name(v: &Verdict) -> &'static str {
    match v {
        Verdict::Pass => "pass",
        Verdict::Warn => "warn",
        Verdict::Fail => "fail",
    }
}

pub async fn fingerprint(
    State(state): State<AppState>,
    Json(req): Json<FingerprintRequest>,
) -> ApiResult<FingerprintResponse> {
    let profile = state
        .profile_repo
        .load(FsPath::new(&req.profile))
        .await
        .map_err(|e| profile_load_failed(&req.profile, &e))?;
    let out = TestFingerprint::new(Arc::new(CdpSessionRuntime::new()))
        .execute(TestFingerprintInput {
            binary: state.binary.clone(),
            profile,
            headful: false,
            geo_sync: req.geo_sync,
        })
        .await
        .map_err(internal)?;
    let probes: Vec<ProbeDto> = out
        .probes
        .into_iter()
        .map(|p| ProbeDto {
            id: p.id,
            passed: p.passed && p.error.is_none(),
            got: p.got,
            expected: p.expected,
            error: p.error,
        })
        .collect();
    let passed = probes.iter().filter(|p| p.passed).count();
    let failed = probes.len() - passed;
    Ok(Json(FingerprintResponse {
        probes,
        passed,
        failed,
    }))
}

pub async fn stealth(
    State(state): State<AppState>,
    Json(req): Json<StealthRequest>,
) -> ApiResult<StealthResponse> {
    let out = ValidateStealth::new(
        Arc::clone(&state.profile_repo),
        Arc::new(CdpSessionRuntime::new()),
    )
    .execute(ValidateStealthInput {
        profile: PathBuf::from(&req.profile),
        binary: state.binary.clone(),
        targets: builtin_targets(req.bot_check_url.as_deref()),
        headful: false,
        geo_sync: req.geo_sync,
    })
    .await
    .map_err(internal)?;
    let results = out
        .results
        .iter()
        .map(|r| StealthResultDto {
            target: r.target.clone(),
            verdict: verdict_name(&r.verdict),
            detail: r.detail.clone(),
            duration_ms: r.duration_ms,
        })
        .collect();
    Ok(Json(StealthResponse { results }))
}
