use anyhow::Result;
use clap::Args;

use crate::presentation::cli::state::CliState;
use crate::presentation::http::AppState;

#[derive(Debug, Args)]
pub struct ServeArgs {
    #[arg(long, default_value = "3000", env = "COSMIUM_PORT")]
    pub port: u16,

    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<std::path::PathBuf>,
}

pub async fn execute(args: ServeArgs, state: &CliState) -> Result<()> {
    let binary = args.binary.unwrap_or_else(|| state.binary.clone());
    let app_state = AppState {
        profile_repo: state.profile_repo.clone(),
        binary,
    };

    println!(
        "  Cosmium server starting on http://localhost:{}",
        args.port
    );
    println!("  Open your browser to start scraping\n");

    crate::presentation::http::serve(app_state, args.port).await
}
