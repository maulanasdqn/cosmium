use std::sync::Arc;

use crate::domain::profile::{ProfileRepository, ProfileResult};

pub struct ListProfiles {
    repo: Arc<dyn ProfileRepository>,
}

impl ListProfiles {
    pub fn new(repo: Arc<dyn ProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self) -> ProfileResult<Vec<String>> {
        self.repo.list().await
    }
}
