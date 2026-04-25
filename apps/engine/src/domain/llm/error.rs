use thiserror::Error;

pub type LlmResult<T> = Result<T, LlmError>;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("OPENROUTER_API_KEY not set")]
    MissingApiKey,

    #[error("transport error")]
    Transport(#[from] reqwest::Error),

    #[error("provider returned status {status}: {body}")]
    Provider { status: u16, body: String },

    #[error("response had no choices")]
    EmptyResponse,

    #[error("malformed response: {0}")]
    Malformed(String),
}
