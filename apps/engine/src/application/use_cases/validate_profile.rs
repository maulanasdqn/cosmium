use std::path::Path;
use std::sync::Arc;

use crate::domain::profile::{Diagnostic, Profile, ProfileRepository, ProfileResult, validation};

pub struct ValidateProfile {
    repo: Arc<dyn ProfileRepository>,
}

pub struct ValidateProfileOutput {
    pub profile: Profile,
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidateProfile {
    pub fn new(repo: Arc<dyn ProfileRepository>) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, path: &Path) -> ProfileResult<ValidateProfileOutput> {
        let profile = self.repo.load(path).await?;
        let diagnostics = validation::validate(&profile);
        Ok(ValidateProfileOutput {
            profile,
            diagnostics,
        })
    }
}
