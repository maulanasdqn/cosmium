pub mod dto;
pub mod handlers;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::CorsLayer;

use crate::domain::profile::ProfileRepository;

#[derive(Clone)]
pub struct AppState {
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub binary: PathBuf,
}

pub fn router(state: AppState) -> Router {
    let api = Router::new()
        .route("/health", get(handlers::health))
        .route("/v1/profiles", get(handlers::list_profiles))
        .route("/v1/profiles/{name}", get(handlers::get_profile))
        .route("/v1/scrape", post(handlers::scrape));

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
