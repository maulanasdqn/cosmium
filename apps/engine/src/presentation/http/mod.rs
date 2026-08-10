pub mod dto;
pub mod handlers;
pub mod handlers_scrape;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
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
}

pub async fn require_api_key(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let provided = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    if provided != state.api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(request).await)
}

pub fn router(state: AppState) -> Router {
    let auth_middleware = axum::middleware::from_fn_with_state(state.clone(), require_api_key);

    let protected = Router::new()
        .route("/v1/profiles", get(handlers::list_profiles))
        .route("/v1/profiles/{name}", get(handlers::get_profile))
        .route("/v1/profiles/generate", post(handlers::generate_profile))
        .route("/v1/profiles/save", post(handlers::save_profile))
        .route("/v1/scrape", post(handlers_scrape::scrape))
        .layer(auth_middleware);

    let api = Router::new()
        .route("/health", get(handlers::health))
        .route("/auth/verify", post(handlers::verify_token))
        .merge(protected);

    Router::new()
        .route("/", get(handlers::index))
        .nest("/api", api)
        .layer(CorsLayer::permissive())
        .with_state(state)
}

pub async fn serve(state: AppState, port: u16) -> anyhow::Result<()> {
    let app = router(state);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://localhost:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
