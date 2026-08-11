use serde::{Deserialize, Serialize};

use super::workflow::WorkflowStep;

#[derive(Debug, Clone)]
pub struct ScrapeRequest {
    pub url: String,
    pub wait_ms: u32,
    pub screenshot: bool,
    pub workflow: Vec<WorkflowStep>,
    pub proxy: Option<ProxyConfig>,
    pub wait_for_api: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub url: String,
}
