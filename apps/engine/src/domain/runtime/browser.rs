use std::path::Path;

use async_trait::async_trait;

use super::error::RuntimeResult;

#[derive(Debug, Clone)]
pub struct LaunchSpec {
    pub flags: Vec<String>,
    pub urls: Vec<String>,
}

#[async_trait]
pub trait BrowserRuntime: Send + Sync {
    async fn launch(&self, binary: &Path, spec: LaunchSpec) -> RuntimeResult<()>;
}
