use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};

use crate::domain::profile::ProfileRepository;
use crate::domain::runtime::{
    BrowserRuntime, browser::LaunchSpec, profile_to_env, profile_to_flags, user_data_dir,
};

pub struct RunBrowser {
    profile_repo: Arc<dyn ProfileRepository>,
    runtime: Arc<dyn BrowserRuntime>,
}

pub struct RunBrowserInput {
    pub profile: PathBuf,
    pub binary: PathBuf,
    pub urls: Vec<String>,
    pub extra_flags: Vec<String>,
}

impl RunBrowser {
    pub fn new(profile_repo: Arc<dyn ProfileRepository>, runtime: Arc<dyn BrowserRuntime>) -> Self {
        Self {
            profile_repo,
            runtime,
        }
    }

    pub async fn execute(&self, input: RunBrowserInput) -> Result<()> {
        let profile = self
            .profile_repo
            .load(&input.profile)
            .await
            .with_context(|| format!("loading {}", input.profile.display()))?;

        let mut flags = profile_to_flags(&profile);
        flags.extend(input.extra_flags);
        let env = profile_to_env(&profile);
        let data_dir = user_data_dir(&profile.name);

        self.runtime
            .launch(
                Path::new(&input.binary),
                LaunchSpec {
                    flags,
                    urls: input.urls,
                    env,
                    user_data_dir: Some(data_dir),
                },
            )
            .await
            .with_context(|| format!("launching {}", input.binary.display()))?;
        Ok(())
    }
}
