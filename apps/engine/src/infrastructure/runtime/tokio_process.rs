use std::path::Path;

use async_trait::async_trait;
use tokio::process::Command;

use crate::domain::runtime::{BrowserRuntime, RuntimeError, RuntimeResult, browser::LaunchSpec};

pub struct TokioProcessRuntime;

impl TokioProcessRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TokioProcessRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BrowserRuntime for TokioProcessRuntime {
    async fn launch(&self, binary: &Path, spec: LaunchSpec) -> RuntimeResult<()> {
        if !binary.exists() {
            return Err(RuntimeError::BinaryNotFound(binary.to_path_buf()));
        }

        if let Some(dir) = spec.user_data_dir.as_ref() {
            std::fs::create_dir_all(dir).map_err(|source| RuntimeError::Spawn {
                binary: binary.to_path_buf(),
                source,
            })?;
        }

        let mut cmd = Command::new(binary);
        for (k, v) in &spec.env {
            cmd.env(k, v);
        }
        cmd.args(&spec.flags);
        if let Some(dir) = spec.user_data_dir.as_ref() {
            cmd.arg(format!("--user-data-dir={}", dir.display()));
        }
        cmd.args(&spec.urls);

        let mut child = cmd.spawn().map_err(|source| RuntimeError::Spawn {
            binary: binary.to_path_buf(),
            source,
        })?;
        let status = child.wait().await.map_err(|source| RuntimeError::Spawn {
            binary: binary.to_path_buf(),
            source,
        })?;
        if !status.success() {
            return Err(RuntimeError::Exit(status.code().unwrap_or(-1)));
        }
        Ok(())
    }
}
