use axum::Json;
use axum::http::StatusCode;

use crate::domain::profile::ProfileError;

use super::dto::ErrorResponse;

pub type ApiError = (StatusCode, Json<ErrorResponse>);
pub type ApiResult<T> = Result<Json<T>, ApiError>;

pub fn api_error(status: StatusCode, message: impl Into<String>) -> ApiError {
    (
        status,
        Json(ErrorResponse {
            error: message.into(),
        }),
    )
}

pub fn internal(e: impl std::fmt::Display) -> ApiError {
    api_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
}

pub fn internal_chain(e: &anyhow::Error) -> ApiError {
    let mut message = e.to_string();
    for cause in e.chain().skip(1) {
        message.push_str(": ");
        message.push_str(&cause.to_string());
    }
    api_error(StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn profile_error(name: &str, e: &ProfileError) -> (StatusCode, String) {
    match e {
        ProfileError::NotFound(_) => (StatusCode::NOT_FOUND, format!("profile not found: {name}")),
        ProfileError::Json { source, .. } => (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("profile {name} is not valid: {source}"),
        ),
        ProfileError::Io { .. } => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("could not read profile {name}"),
        ),
        ProfileError::Invalid(n) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            format!("profile {name} failed validation ({n} error(s))"),
        ),
    }
}

pub fn profile_load_failed(name: &str, e: &ProfileError) -> ApiError {
    let (status, message) = profile_error(name, e);
    api_error(status, message)
}

pub fn sanitize_name(raw: &str) -> Option<String> {
    let name: String = raw
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::{internal_chain, profile_error, sanitize_name};
    use crate::domain::profile::ProfileError;
    use axum::http::StatusCode;

    #[test]
    fn profile_errors_hide_server_paths() {
        let e = ProfileError::NotFound("/srv/secret/profiles/x.json".into());
        let (status, msg) = profile_error("x", &e);
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(!msg.contains("/srv"));
    }

    #[test]
    fn strips_path_characters() {
        assert_eq!(sanitize_name("../etc/passwd").as_deref(), Some("etcpasswd"));
        assert_eq!(sanitize_name("win11_rtx-1").as_deref(), Some("win11_rtx-1"));
        assert_eq!(sanitize_name("../.."), None);
    }

    #[test]
    fn internal_chain_includes_every_cause() {
        let root = anyhow::anyhow!("provider returned status 429: rate limited");
        let wrapped = root.context("LLM chat call failed after retries");
        let (status, body) = internal_chain(&wrapped);
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(body.0.error.contains("after retries"), "{}", body.0.error);
        assert!(body.0.error.contains("429"), "{}", body.0.error);
    }

    #[test]
    fn internal_chain_handles_a_single_error() {
        let (_, body) = internal_chain(&anyhow::anyhow!("plain failure"));
        assert_eq!(body.0.error, "plain failure");
    }
}
