pub mod commands;
pub mod state;

use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::env::Env;

use crate::infrastructure::profile::FsJsonProfileRepository;
use crate::infrastructure::runtime::TokioProcessRuntime;
use commands::profile::ProfileCmd;
use commands::run::RunCmd;
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
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let env = Env::init()?;
    let state = CliState {
        profile_repo: Arc::new(FsJsonProfileRepository::new(env.profiles_dir.clone())),
        runtime: Arc::new(TokioProcessRuntime::new()),
        binary: env.binary.clone(),
    };

    match cli.command {
        Command::Profile(cmd) => commands::profile::execute(cmd, &state).await,
        Command::Run(cmd) => commands::run::execute(cmd, &state).await,
    }
}
