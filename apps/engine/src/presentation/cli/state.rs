use std::path::PathBuf;
use std::sync::Arc;

use crate::domain::profile::ProfileRepository;
use crate::domain::runtime::BrowserRuntime;

#[derive(Clone)]
pub struct CliState {
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub runtime: Arc<dyn BrowserRuntime>,
    pub binary: PathBuf,
}
