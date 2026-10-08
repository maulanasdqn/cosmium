use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::infrastructure::results::{NewResult, ResultStats, ResultSummary, StoredResult};

use super::AppState;
use super::error::{ApiResult, api_error, internal};

#[derive(Debug, Deserialize)]
pub struct CreateResultRequest {
    #[serde(default)]
    pub title: Option<String>,
    pub payload: Value,
    pub result: Value,
    #[serde(default)]
    pub ai_request: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteManyRequest {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ResultList {
    pub results: Vec<ResultSummary>,
}

#[derive(Debug, Serialize)]
pub struct Deleted {
    pub deleted: String,
}

#[derive(Debug, Serialize)]
pub struct DeletedCount {
    pub deleted: usize,
}

fn not_found(id: &str) -> super::error::ApiError {
    api_error(StatusCode::NOT_FOUND, format!("result not found: {id}"))
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateResultRequest>,
) -> ApiResult<ResultSummary> {
    let stored = state
        .results
        .create(NewResult {
            title: req.title,
            payload: req.payload,
            result: req.result,
            ai_request: req.ai_request,
        })
        .await
        .map_err(internal)?;
    Ok(Json(stored.summary()))
}

pub async fn list(State(state): State<AppState>) -> ApiResult<ResultList> {
    let results = state.results.list().await.map_err(internal)?;
    Ok(Json(ResultList { results }))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<StoredResult> {
    state
        .results
        .get(&id)
        .await
        .map_err(internal)?
        .map(Json)
        .ok_or_else(|| not_found(&id))
}

pub async fn set_ai(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(ai): Json<Value>,
) -> ApiResult<ResultSummary> {
    state
        .results
        .set_ai(&id, ai)
        .await
        .map_err(internal)?
        .map(|stored| Json(stored.summary()))
        .ok_or_else(|| not_found(&id))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Deleted> {
    if state.results.delete(&id).await.map_err(internal)? {
        Ok(Json(Deleted { deleted: id }))
    } else {
        Err(not_found(&id))
    }
}

pub async fn delete_many(
    State(state): State<AppState>,
    Json(req): Json<DeleteManyRequest>,
) -> ApiResult<DeletedCount> {
    let deleted = state.results.delete_many(&req.ids).await;
    Ok(Json(DeletedCount { deleted }))
}

pub async fn prune(State(state): State<AppState>) -> ApiResult<DeletedCount> {
    let deleted = state.results.prune().await.map_err(internal)?;
    Ok(Json(DeletedCount { deleted }))
}

pub async fn stats(State(state): State<AppState>) -> ApiResult<ResultStats> {
    state.results.stats().await.map(Json).map_err(internal)
}
