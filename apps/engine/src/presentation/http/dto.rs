use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
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
    pub proxy: Option<String>,
    #[serde(default)]
    pub include_html: bool,
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
    pub elapsed_ms: u64,
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
    pub code: String,
    pub message: String,
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
