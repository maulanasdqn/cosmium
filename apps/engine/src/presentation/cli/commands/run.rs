use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::application::use_cases::run_browser::{RunBrowser, RunBrowserInput};
use crate::presentation::cli::state::CliState;

#[derive(Debug, Args)]
pub struct RunCmd {
    #[arg(long)]
    pub profile: PathBuf,
    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<PathBuf>,
    #[arg(long = "flag")]
    pub flags: Vec<String>,
    pub urls: Vec<String>,
}

pub async fn execute(cmd: RunCmd, state: &CliState) -> Result<()> {
    let uc = RunBrowser::new(state.profile_repo.clone(), state.runtime.clone());
    uc.execute(RunBrowserInput {
        profile: cmd.profile,
        binary: cmd.binary.unwrap_or_else(|| state.binary.clone()),
        urls: cmd.urls,
        extra_flags: cmd.flags,
    })
    .await
}
