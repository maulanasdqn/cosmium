use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::use_cases::format_scrape::{FormatScrape, FormatScrapeInput};

use super::AppState;
use super::error::{ApiResult, internal};
use super::handlers_llm::require_llm;

#[derive(Debug, Deserialize)]
pub struct FormatRequest {
    pub url: String,
    #[serde(default)]
    pub instruction: Option<String>,
    pub data: Value,
}

#[derive(Debug, Serialize)]
pub struct FormatResponse {
    pub data: Value,
    pub model: String,
}

pub async fn format(
    State(state): State<AppState>,
    Json(req): Json<FormatRequest>,
) -> ApiResult<FormatResponse> {
    let llm = require_llm(&state)?;
    let out = FormatScrape::new(llm, state.llm_model.clone())
        .execute(FormatScrapeInput {
            url: req.url,
            instruction: req.instruction,
            content: req.data,
        })
        .await
        .map_err(internal)?;
    Ok(Json(FormatResponse {
        data: out.data,
        model: out.model,
    }))
}
