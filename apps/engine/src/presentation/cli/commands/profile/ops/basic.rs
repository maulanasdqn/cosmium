use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use super::io::{count, print_diagnostics};
use crate::application::use_cases::list_profiles::ListProfiles;
use crate::application::use_cases::validate_profile::ValidateProfile;
use crate::presentation::cli::state::CliState;

pub async fn validate(target: PathBuf, strict: bool, state: &CliState) -> Result<()> {
    let uc = ValidateProfile::new(state.profile_repo.clone());
    let out = uc
        .execute(&target)
        .await
        .with_context(|| format!("validating {}", target.display()))?;

    println!("profile: {}", out.profile.name);
    print_diagnostics(&out.diagnostics);

    let (errors, warnings) = count(&out.diagnostics);
    if errors > 0 || (strict && warnings > 0) {
        bail!("profile has unresolved diagnostics");
    }
    Ok(())
}

pub async fn show(target: PathBuf, state: &CliState) -> Result<()> {
    let profile = state
        .profile_repo
        .load(&target)
        .await
        .with_context(|| format!("loading {}", target.display()))?;
    println!("{}", serde_json::to_string_pretty(&profile)?);
    Ok(())
}

pub async fn list(state: &CliState) -> Result<()> {
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
