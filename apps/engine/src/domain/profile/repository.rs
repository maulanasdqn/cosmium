use std::path::Path;

use async_trait::async_trait;

use super::Profile;
use super::error::ProfileResult;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn load(&self, path: &Path) -> ProfileResult<Profile>;
    async fn list(&self) -> ProfileResult<Vec<String>>;
}
