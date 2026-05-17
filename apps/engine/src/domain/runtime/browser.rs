use std::path::{Path, PathBuf};

use async_trait::async_trait;

use super::error::RuntimeResult;

#[derive(Debug, Clone, Default)]
pub struct LaunchSpec {
    pub flags: Vec<String>,
    pub urls: Vec<String>,
    pub env: Vec<(String, String)>,
    pub user_data_dir: Option<PathBuf>,
}

#[async_trait]
pub trait BrowserRuntime: Send + Sync {
    async fn launch(&self, binary: &Path, spec: LaunchSpec) -> RuntimeResult<()>;
}
