pub mod auth;
pub mod dto;
pub mod dto_platform;
pub mod error;
pub mod handlers;
pub mod handlers_llm;
pub mod handlers_profiles;
pub mod handlers_scrape;
pub mod handlers_tests;
pub mod summary;
pub mod ui;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;

use crate::domain::llm::LlmClient;
use crate::domain::profile::ProfileRepository;

#[derive(Clone)]
pub struct AppState {
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub binary: PathBuf,
    pub profiles_dir: PathBuf,
    pub api_key: String,
    pub llm: Option<Arc<dyn LlmClient>>,
    pub llm_model: String,
    pub ui_dir: Option<PathBuf>,
}

pub fn router(state: AppState) -> Router {
    let auth_middleware =
        axum::middleware::from_fn_with_state(state.clone(), auth::require_api_key);

    let protected = Router::new()
        .route("/v1/profiles", get(handlers::list_profiles))
        .route("/v1/profiles/summaries", get(handlers_profiles::summaries))
        .route("/v1/profiles/generate", post(handlers::generate_profile))
        .route("/v1/profiles/save", post(handlers::save_profile))
        .route(
            "/v1/profiles/validate",
            post(handlers_profiles::validate_body),
        )
        .route(
            "/v1/profiles/{name}",
            get(handlers::get_profile).delete(handlers_profiles::delete),
        )
        .route(
            "/v1/profiles/{name}/validate",
            get(handlers_profiles::validate_named),
        )
        .route("/v1/profiles/{name}/repair", post(handlers_llm::repair))
        .route("/v1/profiles/{name}/mutate", post(handlers_llm::mutate))
        .route("/v1/scrape", post(handlers_scrape::scrape))
        .route("/v1/tests/fingerprint", post(handlers_tests::fingerprint))
        .route("/v1/tests/stealth", post(handlers_tests::stealth))
        .layer(auth_middleware);

    let api = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/verify", post(auth::verify_token))
        .merge(protected)
        .fallback(api_not_found);

    let ui_dir = state.ui_dir.clone();
    ui::attach(Router::new().nest("/api", api), ui_dir.as_deref())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn api_not_found() -> error::ApiError {
    error::api_error(StatusCode::NOT_FOUND, "not found")
}

pub async fn serve(state: AppState, port: u16) -> anyhow::Result<()> {
    let app = router(state);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://localhost:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
