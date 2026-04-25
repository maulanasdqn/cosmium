use std::path::PathBuf;

use thiserror::Error;

pub type ProfileResult<T> = Result<T, ProfileError>;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("profile not found: {0}")]
    NotFound(PathBuf),

    #[error("io error reading {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("json parse error in {path}")]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("profile failed coherence validation ({0} error(s))")]
    Invalid(usize),
}
