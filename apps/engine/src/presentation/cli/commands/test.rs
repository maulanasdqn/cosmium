use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::{Args, Subcommand};

use crate::application::use_cases::test_fingerprint::{
    TestFingerprint, TestFingerprintInput,
};
use crate::presentation::cli::state::CliState;

#[derive(Debug, Subcommand)]
pub enum TestCmd {
    Fingerprint(FingerprintArgs),
}

#[derive(Debug, Args)]
pub struct FingerprintArgs {
    #[arg(long)]
    pub profile: PathBuf,
    #[arg(long, env = "COSMIUM_BINARY")]
    pub binary: Option<PathBuf>,
}

pub async fn execute(cmd: TestCmd, state: &CliState) -> Result<()> {
    match cmd {
        TestCmd::Fingerprint(args) => run_fingerprint(args, state).await,
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
    let out = uc
        .execute(TestFingerprintInput { binary, profile })
        .await?;

    let mut passed = 0usize;
    let mut failed = 0usize;
    println!("{:<22} {:<8} {}", "PROBE", "RESULT", "VALUE");
    println!("{:-<22} {:-<8} {:-<40}", "", "", "");
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
