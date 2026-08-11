use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};

use crate::application::use_cases::test_fingerprint::{TestFingerprint, TestFingerprintInput};
use crate::application::use_cases::validate_stealth::targets::builtin_targets;
use crate::application::use_cases::validate_stealth::{ValidateStealth, ValidateStealthInput};
use crate::domain::scraping::validation::Verdict;
use crate::presentation::cli::state::CliState;

#[derive(Debug, Subcommand)]
pub enum TestCmd {
    Fingerprint(FingerprintArgs),
    Stealth(StealthArgs),
}

#[derive(Debug, Args)]
pub struct FingerprintArgs {
    #[arg(long)]
    pub profile: PathBuf,
    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct StealthArgs {
    #[arg(long)]
    pub profile: PathBuf,
    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<PathBuf>,
    #[arg(long)]
    pub bot_check_url: Option<String>,
    #[arg(long)]
    pub headful: bool,
    #[arg(long)]
    pub json: bool,
}

pub async fn execute(cmd: TestCmd, state: &CliState) -> Result<()> {
    match cmd {
        TestCmd::Fingerprint(args) => run_fingerprint(args, state).await,
        TestCmd::Stealth(args) => run_stealth(args, state).await,
    }
}

async fn run_fingerprint(args: FingerprintArgs, state: &CliState) -> Result<()> {
    let profile = state
        .profile_repo
        .load(&args.profile)
        .await
        .with_context(|| format!("loading {}", args.profile.display()))?;
    let binary = args.binary.unwrap_or_else(|| state.binary.clone());

    let uc = TestFingerprint::new();
    let out = uc.execute(TestFingerprintInput { binary, profile }).await?;

    let mut passed = 0usize;
    let mut failed = 0usize;
    println!("{:<22} {:<8} VALUE", "PROBE", "RESULT");
    println!("{} {} {}", "-".repeat(22), "-".repeat(8), "-".repeat(40));
    for r in &out.probes {
        let tag = match (&r.error, r.passed) {
            (Some(e), _) => format!("ERROR  ({e})"),
            (None, true) => "PASS".into(),
            (None, false) => format!("FAIL  expected={}", r.expected),
        };
        if r.error.is_some() || !r.passed {
            failed += 1;
        } else {
            passed += 1;
        }
        println!("{:<22} {:<8} {}", r.id, tag, r.got);
    }
    println!();
    println!("passed={passed} failed={failed}");
    if failed > 0 {
        bail!("fingerprint test had failures");
    }
    Ok(())
}

async fn run_stealth(args: StealthArgs, state: &CliState) -> Result<()> {
    let session = std::sync::Arc::new(crate::infrastructure::runtime::CdpSessionRuntime::new());
    let binary = args.binary.unwrap_or_else(|| state.binary.clone());
    let targets = builtin_targets(args.bot_check_url.as_deref());

    let uc = ValidateStealth::new(state.profile_repo.clone(), session);
    let out = uc
        .execute(ValidateStealthInput {
            profile: args.profile,
            binary,
            targets,
            headful: args.headful,
        })
        .await?;

    if args.json {
        let json = serde_json::to_string_pretty(&out.results)?;
        println!("{json}");
    } else {
        println!(
            "{:<14} {:<6} {:<40} {:>8}",
            "TARGET", "PASS", "DETAIL", "TIME"
        );
        println!("{}", "-".repeat(72));
        for r in &out.results {
            let icon = match r.verdict {
                Verdict::Pass => "✓",
                Verdict::Warn => "~",
                Verdict::Fail => "✗",
            };
            println!(
                "{:<14} {:<6} {:<40} {:>6}ms",
                r.target, icon, r.detail, r.duration_ms
            );
        }
        let pass_count = out
            .results
            .iter()
            .filter(|r| r.verdict == Verdict::Pass)
            .count();
        let fail_count = out
            .results
            .iter()
            .filter(|r| r.verdict == Verdict::Fail)
            .count();
        let warn_count = out
            .results
            .iter()
            .filter(|r| r.verdict == Verdict::Warn)
            .count();
        println!();
        println!("pass={pass_count} warn={warn_count} fail={fail_count}");
    }

    let has_fail = out.results.iter().any(|r| r.verdict == Verdict::Fail);
    if has_fail {
        bail!("stealth validation had failures");
    }
    Ok(())
}
