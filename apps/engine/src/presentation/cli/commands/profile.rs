use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Subcommand;

use crate::application::use_cases::list_profiles::ListProfiles;
use crate::application::use_cases::validate_profile::ValidateProfile;
use crate::domain::profile::Severity;
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
}

pub async fn execute(cmd: ProfileCmd, state: &CliState) -> Result<()> {
    match cmd {
        ProfileCmd::Validate { target, strict } => validate(target, strict, state).await,
        ProfileCmd::Show { target } => show(target, state).await,
        ProfileCmd::List => list(state).await,
    }
}

async fn validate(target: PathBuf, strict: bool, state: &CliState) -> Result<()> {
    let uc = ValidateProfile::new(state.profile_repo.clone());
    let out = uc
        .execute(&target)
        .await
        .with_context(|| format!("validating {}", target.display()))?;

    println!("profile: {}", out.profile.name);

    if out.diagnostics.is_empty() {
        println!("  coherent (0 diagnostics)");
        return Ok(());
    }

    let mut errors = 0usize;
    let mut warnings = 0usize;
    for d in &out.diagnostics {
        let tag = match d.severity {
            Severity::Error => {
                errors += 1;
                "ERROR"
            }
            Severity::Warning => {
                warnings += 1;
                "WARN "
            }
        };
        println!("  [{tag}] {:<40} {}", d.field, d.message);
    }
    println!("  {errors} error(s), {warnings} warning(s)");

    if errors > 0 || (strict && warnings > 0) {
        bail!("profile has unresolved diagnostics");
    }
    Ok(())
}

async fn show(target: PathBuf, state: &CliState) -> Result<()> {
    let profile = state
        .profile_repo
        .load(&target)
        .await
        .with_context(|| format!("loading {}", target.display()))?;
    let s = serde_json::to_string_pretty(&profile)?;
    println!("{s}");
    Ok(())
}

async fn list(state: &CliState) -> Result<()> {
    let uc = ListProfiles::new(state.profile_repo.clone());
    let names = uc.execute().await?;
    if names.is_empty() {
        println!("(no profiles found)");
    } else {
        for n in names {
            println!("{n}");
        }
    }
    Ok(())
}
