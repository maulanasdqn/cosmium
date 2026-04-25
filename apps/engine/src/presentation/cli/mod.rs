pub mod commands;
pub mod state;

use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::env::Env;

use crate::infrastructure::llm::OpenRouterClient;
use crate::infrastructure::llm::openrouter::OpenRouterConfig;
use crate::infrastructure::profile::FsJsonProfileRepository;
use crate::infrastructure::runtime::TokioProcessRuntime;
use commands::build::BuildCmd;
use commands::profile::ProfileCmd;
use commands::run::RunCmd;
use commands::test::TestCmd;
use state::CliState;

#[derive(Debug, Parser)]
#[command(name = "cosmium", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Profile(ProfileCmd),
    Run(RunCmd),
    Build(BuildCmd),
    #[command(subcommand)]
    Test(TestCmd),
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let env = Env::init()?;
    let llm = env.openrouter.api_key.as_ref().map(|key| {
        Arc::new(OpenRouterClient::new(OpenRouterConfig {
            api_key: key.clone(),
            base_url: env.openrouter.base_url.clone(),
            referer: env.openrouter.referer.clone(),
            title: env.openrouter.title.clone(),
        })) as Arc<dyn crate::domain::llm::LlmClient>
    });
    let state = CliState {
        profile_repo: Arc::new(FsJsonProfileRepository::new(env.profiles_dir.clone())),
        runtime: Arc::new(TokioProcessRuntime::new()),
        llm,
        llm_model: env.openrouter.model.clone(),
        binary: env.binary.clone(),
    };

    match cli.command {
        Command::Profile(cmd) => commands::profile::execute(cmd, &state).await,
        Command::Run(cmd) => commands::run::execute(cmd, &state).await,
        Command::Build(cmd) => commands::build::execute(cmd).await,
        Command::Test(cmd) => commands::test::execute(cmd, &state).await,
    }
}
