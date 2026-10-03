use std::path::{Path, PathBuf};

use axum::Router;
use axum::routing::get;
use tower_http::services::{ServeDir, ServeFile};

use super::AppState;
use super::handlers::index;

pub fn ui_index(dir: &Path) -> Option<PathBuf> {
    let index = dir.join("index.html");
    index.is_file().then_some(index)
}

pub fn attach(router: Router<AppState>, ui_dir: Option<&Path>) -> Router<AppState> {
    match ui_dir.and_then(|dir| ui_index(dir).map(|index| (dir, index))) {
        Some((dir, index)) => {
            tracing::info!(dir = %dir.display(), "serving UI");
            router.fallback_service(ServeDir::new(dir).fallback(ServeFile::new(index)))
        }
        None => router.route("/", get(index)),
    }
}

#[cfg(test)]
mod tests {
    use super::ui_index;

    #[test]
    fn requires_an_index_html() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        assert!(ui_index(dir.path()).is_none());
        std::fs::write(dir.path().join("index.html"), "<html></html>")
            .unwrap_or_else(|e| panic!("write: {e}"));
        assert_eq!(ui_index(dir.path()), Some(dir.path().join("index.html")));
    }
}
