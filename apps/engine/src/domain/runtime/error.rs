use std::path::PathBuf;

use thiserror::Error;

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("binary not found: {0}")]
    BinaryNotFound(PathBuf),

    #[error("spawn failed for {binary}")]
    Spawn {
        binary: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("non-zero exit: {0}")]
    Exit(i32),
}
