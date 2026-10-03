use serde::{Deserialize, Serialize};

use super::dto::DiagnosticDto;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub binary_found: bool,
    pub llm_configured: bool,
}

#[derive(Debug, Default, Serialize)]
pub struct ProfileSummary {
    pub name: String,
    pub navigator_platform: Option<String>,
    pub client_hints_platform: Option<String>,
    pub chrome_version: Option<String>,
    pub user_agent: Option<String>,
    pub gpu_renderer: Option<String>,
    pub timezone: Option<String>,
    pub languages: Option<Vec<String>>,
    pub screen: Option<String>,
    pub device_pixel_ratio: Option<f32>,
    pub errors: Option<usize>,
    pub warnings: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProfileSummariesResponse {
    pub profiles: Vec<ProfileSummary>,
}

#[derive(Debug, Serialize)]
pub struct DeletedResponse {
    pub deleted: String,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticsResponse {
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub profile: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ProfileWithDiagnostics {
    pub profile: serde_json::Value,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Deserialize)]
pub struct MutateRequest {
    pub count: usize,
    pub hint: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MutateResponse {
    pub variants: Vec<ProfileWithDiagnostics>,
}

#[derive(Debug, Deserialize)]
pub struct FingerprintRequest {
    pub profile: String,
    #[serde(default)]
    pub geo_sync: bool,
}

#[derive(Debug, Serialize)]
pub struct ProbeDto {
    pub id: String,
    pub passed: bool,
    pub got: String,
    pub expected: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FingerprintResponse {
    pub probes: Vec<ProbeDto>,
    pub passed: usize,
    pub failed: usize,
}

#[derive(Debug, Deserialize)]
pub struct StealthRequest {
    pub profile: String,
    pub bot_check_url: Option<String>,
    #[serde(default)]
    pub geo_sync: bool,
}

#[derive(Debug, Serialize)]
pub struct StealthResultDto {
    pub target: String,
    pub verdict: &'static str,
    pub detail: String,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct StealthResponse {
    pub results: Vec<StealthResultDto>,
}
