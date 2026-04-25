mod parse;
mod probe;

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use tokio::fs;
use tokio::process::Command;

use crate::domain::profile::Profile;
use crate::domain::runtime::profile_to_flags;

pub use probe::ProbeResult;

pub struct TestFingerprint;

pub struct TestFingerprintInput {
    pub binary: PathBuf,
    pub profile: Profile,
}

pub struct TestFingerprintOutput {
    pub probes: Vec<ProbeResult>,
}

impl TestFingerprint {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self, input: TestFingerprintInput) -> Result<TestFingerprintOutput> {
        if !input.binary.exists() {
            bail!("binary not found: {}", input.binary.display());
        }
        let probes = probe::for_profile(&input.profile);
        let temp = tempfile::tempdir()?;
        let html_path = temp.path().join("probes.html");
        fs::write(&html_path, probe::render_html(&probes))
            .await
            .context("writing probes.html")?;

        let mut flags = profile_to_flags(&input.profile);
        flags.push("--headless=new".into());
        flags.push("--no-sandbox".into());
        flags.push("--dump-dom".into());
        flags.push(format!("file://{}", html_path.display()));

        let output = Command::new(&input.binary)
            .args(&flags)
            .output()
            .await
            .context("launching binary")?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("chrome exit {}: {}", output.status, stderr);
        }
        let dom = String::from_utf8_lossy(&output.stdout);
        let results = parse::extract(&dom, &probes)?;
        Ok(TestFingerprintOutput { probes: results })
    }
}

impl Default for TestFingerprint {
    fn default() -> Self {
        Self::new()
    }
}
