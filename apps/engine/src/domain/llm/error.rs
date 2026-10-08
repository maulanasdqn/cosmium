use thiserror::Error;

pub type LlmResult<T> = Result<T, LlmError>;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("OPENROUTER_API_KEY not set")]
    MissingApiKey,

    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("provider returned status {status}: {body}")]
    Provider { status: u16, body: String },

    #[error("request timed out after {seconds}s")]
    Timeout { seconds: u64 },

    #[error("response had no choices")]
    EmptyResponse,

    #[error("malformed response: {0}")]
    Malformed(String),
}
