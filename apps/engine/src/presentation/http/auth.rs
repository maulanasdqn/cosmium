use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

use super::AppState;
use super::error::{ApiError, api_error};

pub fn keys_match(provided: &str, expected: &str) -> bool {
    let a = provided.as_bytes();
    let b = expected.as_bytes();
    let mut diff = a.len() ^ b.len();
    for i in 0..a.len().max(b.len()) {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        diff |= usize::from(x ^ y);
    }
    diff == 0
}

fn provided_key(headers: &HeaderMap) -> &str {
    headers
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
}

fn unauthorized() -> ApiError {
    api_error(StatusCode::UNAUTHORIZED, "unauthorized")
}

pub async fn require_api_key(
    State(state): State<AppState>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, ApiError> {
    if keys_match(provided_key(request.headers()), &state.api_key) {
        Ok(next.run(request).await)
    } else {
        Err(unauthorized())
    }
}

pub async fn verify_token(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    if keys_match(provided_key(&headers), &state.api_key) {
        Ok(StatusCode::OK)
    } else {
        Err(unauthorized())
    }
}

#[cfg(test)]
mod tests {
    use super::keys_match;

    #[test]
    fn compares_whole_keys() {
        assert!(keys_match("secret", "secret"));
        assert!(!keys_match("secret", "secreT"));
        assert!(!keys_match("secret", "secret2"));
        assert!(!keys_match("", "secret"));
    }
}
