use std::path::PathBuf;

use anyhow::{Context, Result};

use super::io::{print_diagnostics, resolve_output, write_profile};
use crate::application::use_cases::generate_profile::{GenerateProfile, GenerateProfileInput};
use crate::application::use_cases::mutate_profile::{MutateProfile, MutateProfileInput};
use crate::application::use_cases::repair_profile::{RepairProfile, RepairProfileInput};
use crate::domain::profile::Severity;
use crate::presentation::cli::state::CliState;

pub async fn generate(
    persona: String,
    name: String,
    output: Option<PathBuf>,
    save: bool,
    state: &CliState,
) -> Result<()> {
    let llm = state
        .llm
        .clone()
        .context("OPENROUTER_API_KEY not set — generate requires LLM")?;
    let uc = GenerateProfile::new(llm, state.llm_model.clone());
    let out = uc
        .execute(GenerateProfileInput {
            persona,
            name: name.clone(),
        })
        .await?;

    println!("profile: {}", out.profile.name);
    print_diagnostics(&out.diagnostics);

    let dest = resolve_output(output, save, &name)?;
    write_profile(&out.profile, &dest).await?;
    Ok(())
}

pub async fn repair(target: PathBuf, output: Option<PathBuf>, state: &CliState) -> Result<()> {
    let llm = state
        .llm
        .clone()
        .context("OPENROUTER_API_KEY not set — repair requires LLM")?;
    let profile = state
        .profile_repo
        .load(&target)
        .await
        .with_context(|| format!("loading {}", target.display()))?;
    let diags = crate::domain::profile::validation::validate(&profile);
    if diags.iter().all(|d| d.severity != Severity::Error) {
        println!("nothing to repair");
        return Ok(());
    }
    let uc = RepairProfile::new(llm, state.llm_model.clone());
    let out = uc
        .execute(RepairProfileInput {
            profile,
            diagnostics: diags,
        })
        .await?;
    print_diagnostics(&out.diagnostics);
    let dest = output.unwrap_or(target);
    write_profile(&out.profile, &dest).await?;
    Ok(())
}

pub async fn mutate(
    target: PathBuf,
    count: usize,
    hint: Option<String>,
    output_dir: Option<PathBuf>,
    save: bool,
    state: &CliState,
) -> Result<()> {
    let llm = state
        .llm
        .clone()
        .context("OPENROUTER_API_KEY not set — mutate requires LLM")?;
    let reference = state
        .profile_repo
        .load(&target)
        .await
        .with_context(|| format!("loading {}", target.display()))?;
    let uc = MutateProfile::new(llm, state.llm_model.clone());
    let out = uc
        .execute(MutateProfileInput {
            reference,
            count,
            hint,
        })
        .await?;

    let dir = resolve_dir(output_dir, save)?;
    for v in out.variants {
        println!("variant: {}", v.profile.name);
        print_diagnostics(&v.diagnostics);
        let dest = dir.join(format!("{}.json", v.profile.name));
        write_profile(&v.profile, &dest).await?;
    }
    Ok(())
}

fn resolve_dir(output_dir: Option<PathBuf>, save: bool) -> Result<PathBuf> {
    if let Some(p) = output_dir {
        return Ok(p);
    }
    if save {
        let env = config::env::Env::init()?;
        return Ok(env.profiles_dir);
    }
    anyhow::bail!("specify --output-dir or --save");
}
