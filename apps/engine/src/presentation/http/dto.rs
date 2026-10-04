use serde::{Deserialize, Serialize};

use crate::domain::scraping::workflow::WorkflowStep;

#[derive(Debug, Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent opt-in flags of the JSON request body"
)]
pub struct ScrapeRequest {
    pub url: String,
    pub profile: String,
    #[serde(default)]
    pub screenshot: bool,
    #[serde(default)]
    pub headful: bool,
    #[serde(default)]
    pub wait_ms: u32,
    #[serde(default)]
    pub extract: Vec<String>,
    pub script: Option<String>,
    #[serde(default)]
    pub workflow: Vec<WorkflowStep>,
    pub proxy: Option<String>,
    #[serde(default)]
    pub proxies: Vec<String>,
    #[serde(default)]
    pub proxy_rotation: Option<String>,
    #[serde(default)]
    pub include_html: bool,
    pub wait_for_api: Option<String>,
    #[serde(default)]
    pub retries: u32,
    #[serde(default)]
    pub geo_sync: bool,
}

#[derive(Debug, Serialize)]
pub struct ScrapeResponse {
    pub url: String,
    pub final_url: String,
    pub http_status: i32,
    pub html_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    pub user_agent: String,
    pub cookies_count: usize,
    pub blocked: bool,
    pub extracted: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_base64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_used: Option<String>,
    pub elapsed_ms: u64,
    pub attempts: u32,
}

#[derive(Debug, Serialize)]
pub struct ProfileListResponse {
    pub profiles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateProfileRequest {
    pub persona: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateProfileResponse {
    pub profile: serde_json::Value,
    pub diagnostics: Vec<DiagnosticDto>,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticDto {
    pub severity: String,
    pub field: String,
    pub message: String,
}

impl From<&crate::domain::profile::Diagnostic> for DiagnosticDto {
    fn from(d: &crate::domain::profile::Diagnostic) -> Self {
        let severity = match d.severity {
            crate::domain::profile::Severity::Error => "error",
            crate::domain::profile::Severity::Warning => "warning",
        };
        Self {
            severity: severity.to_owned(),
            field: d.field.to_owned(),
            message: d.message.clone(),
        }
    }
}

pub fn diagnostics_dto(diags: &[crate::domain::profile::Diagnostic]) -> Vec<DiagnosticDto> {
    diags.iter().map(DiagnosticDto::from).collect()
}

#[derive(Debug, Deserialize)]
pub struct SaveProfileRequest {
    pub name: String,
    pub profile: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SaveProfileResponse {
    pub saved: String,
}
