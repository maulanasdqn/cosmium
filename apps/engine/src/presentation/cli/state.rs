use std::path::PathBuf;
use std::sync::Arc;

use crate::domain::llm::LlmClient;
use crate::domain::profile::ProfileRepository;
use crate::domain::runtime::BrowserRuntime;

#[derive(Clone)]
pub struct CliState {
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub runtime: Arc<dyn BrowserRuntime>,
    pub llm: Option<Arc<dyn LlmClient>>,
    pub llm_model: String,
    pub binary: PathBuf,
}
