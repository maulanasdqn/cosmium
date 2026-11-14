pub mod auth;
pub mod dto;
pub mod dto_platform;
pub mod error;
pub mod handlers;
pub mod handlers_format;
pub mod handlers_llm;
pub mod handlers_profiles;
pub mod handlers_results;
pub mod handlers_scrape;
pub mod handlers_tests;
mod shutdown;
pub mod summary;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use tower_http::cors::CorsLayer;

use crate::domain::llm::LlmClient;
use crate::domain::profile::ProfileRepository;
use crate::infrastructure::scraping::BrowserPool;

#[derive(Clone)]
pub struct AppState {
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub binary: PathBuf,
    pub profiles_dir: PathBuf,
    pub api_key: String,
    pub llm: Option<Arc<dyn LlmClient>>,
    pub llm_model: String,
    pub results: Arc<crate::infrastructure::results::ResultStore>,
    pub browsers: Arc<BrowserPool>,
}

const RESULT_BODY_LIMIT: usize = 64 * 1024 * 1024;

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
        .route("/v1/scrape/format", post(handlers_format::format))
        .route(
            "/v1/results",
            get(handlers_results::list)
                .post(handlers_results::create)
                .delete(handlers_results::delete_many)
                .layer(axum::extract::DefaultBodyLimit::max(RESULT_BODY_LIMIT)),
        )
        .route("/v1/results/stats", get(handlers_results::stats))
        .route("/v1/results/prune", post(handlers_results::prune))
        .route(
            "/v1/results/{id}",
            get(handlers_results::get).delete(handlers_results::delete),
        )
        .route("/v1/results/{id}/ai", put(handlers_results::set_ai))
        .route("/v1/tests/fingerprint", post(handlers_tests::fingerprint))
        .route("/v1/tests/stealth", post(handlers_tests::stealth))
        .layer(auth_middleware);

    let api = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/verify", post(auth::verify_token))
        .merge(protected)
        .fallback(api_not_found);

    Router::new()
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn api_not_found() -> error::ApiError {
    error::api_error(StatusCode::NOT_FOUND, "not found")
}

pub async fn serve(state: AppState, port: u16) -> anyhow::Result<()> {
    let browsers = Arc::clone(&state.browsers);
    let app = router(state);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://localhost:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let served = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::signal())
        .await;
    tracing::info!("closing pooled browsers");
    browsers.shutdown().await;
    served?;
    Ok(())
}
