use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use tokio::fs;

use crate::domain::profile::{Diagnostic, Profile, Severity};

pub fn print_diagnostics(diagnostics: &[Diagnostic]) {
    if diagnostics.is_empty() {
        println!("  coherent (0 diagnostics)");
        return;
    }
    let (errors, warnings) = count(diagnostics);
    for d in diagnostics {
        let tag = match d.severity {
            Severity::Error => "ERROR",
            Severity::Warning => "WARN ",
        };
        println!("  [{tag}] {:<40} {}", d.field, d.message);
    }
    println!("  {errors} error(s), {warnings} warning(s)");
}

pub fn count(diagnostics: &[Diagnostic]) -> (usize, usize) {
    let e = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let w = diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Warning)
        .count();
    (e, w)
}

pub fn resolve_output(output: Option<PathBuf>, save: bool, name: &str) -> Result<PathBuf> {
    if let Some(p) = output {
        return Ok(p);
    }
    if save {
        let env = config::env::Env::init()?;
        return Ok(env.profiles_dir.join(format!("{name}.json")));
    }
    bail!("specify --output or --save");
}

pub async fn write_profile(p: &Profile, dest: &Path) -> Result<()> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).await.ok();
    }
    let json = serde_json::to_string_pretty(p)?;
    fs::write(dest, json).await?;
    println!("wrote {}", dest.display());
    Ok(())
}
