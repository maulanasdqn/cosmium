use thiserror::Error;

use crate::domain::runtime::RuntimeError;

pub type ScrapeResult<T> = Result<T, ScrapeError>;

#[derive(Debug, Error)]
pub enum ScrapeError {
    #[error("browser connection failed: {0}")]
    Connection(String),

    #[error("navigation timed out after {0}ms")]
    NavigationTimeout(u64),

    #[error("page blocked: status={status}, url={url}")]
    Blocked { status: i32, url: String },

    #[error("workflow step failed: {0}")]
    WorkflowFailed(String),

    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}
