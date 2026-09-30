mod ops;

use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use crate::presentation::cli::state::CliState;

#[derive(Debug, Subcommand)]
pub enum ProfileCmd {
    Validate {
        target: PathBuf,
        #[arg(long)]
        strict: bool,
    },
    Show {
        target: PathBuf,
    },
    List,
    Generate {
        #[arg(long)]
        persona: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long)]
        save: bool,
    },
    Repair {
        target: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Mutate {
        target: PathBuf,
        #[arg(long, default_value_t = 5)]
        count: usize,
        #[arg(long)]
        hint: Option<String>,
        #[arg(long)]
        output_dir: Option<PathBuf>,
        #[arg(long)]
        save: bool,
    },
}

pub async fn execute(cmd: ProfileCmd, state: &CliState) -> Result<()> {
    match cmd {
        ProfileCmd::Validate { target, strict } => ops::validate(target, strict, state).await,
        ProfileCmd::Show { target } => ops::show(target, state).await,
        ProfileCmd::List => ops::list(state).await,
        ProfileCmd::Generate {
            persona,
            name,
            output,
            save,
        } => ops::generate(persona, name, output, save, state).await,
        ProfileCmd::Repair { target, output } => ops::repair(target, output, state).await,
        ProfileCmd::Mutate {
            target,
            count,
            hint,
            output_dir,
            save,
        } => ops::mutate(target, count, hint, output_dir, save, state).await,
    }
}
