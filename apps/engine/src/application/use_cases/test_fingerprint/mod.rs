mod matcher;
mod parse;
pub(crate) mod probe;
mod probe_lies;
mod probe_render;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::fs;

use crate::application::use_cases::stealth_config::build_stealth_config;
use crate::domain::profile::Profile;
use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::{profile_to_env, profile_to_flags};
use crate::domain::scraping::port::BrowserSession;
use crate::infrastructure::scraping::ChromiumScraper;

pub use probe::ProbeResult;

const READ_RESULTS: &str = "await new Promise((res) => { const t = () => { const o = document.getElementById('out'); if (o && o.dataset.done) { res(o.textContent); } else { setTimeout(t, 50); } }; t(); })";

pub struct TestFingerprint {
    session: Arc<dyn BrowserSession>,
}

pub struct TestFingerprintInput {
    pub binary: PathBuf,
    pub profile: Profile,
    pub headful: bool,
    pub geo_sync: bool,
}

pub struct TestFingerprintOutput {
    pub probes: Vec<ProbeResult>,
}

impl TestFingerprint {
    pub fn new(session: Arc<dyn BrowserSession>) -> Self {
        Self { session }
    }

    pub async fn execute(&self, mut input: TestFingerprintInput) -> Result<TestFingerprintOutput> {
        if input.geo_sync {
            if let Some(tz) = crate::infrastructure::geo::exit_timezone(None).await {
                input.profile.locale.timezone = tz;
            }
        }
        let probes = probe::for_profile(&input.profile);
        let temp = tempfile::tempdir()?;
        let html_path = temp.path().join("probes.html");
        fs::write(&html_path, probe_render::render_html(&probes))
            .await
            .context("writing probes.html")?;

        let mut flags = profile_to_flags(&input.profile);
        if !input.headful {
            flags.push("--headless=new".into());
        }
        let endpoint = self
            .session
            .launch_with_cdp(
                Path::new(&input.binary),
                LaunchSpec {
                    flags,
                    urls: Vec::new(),
                    env: profile_to_env(&input.profile),
                    user_data_dir: Some(temp.path().join("profile")),
                    fontconfig: Some(crate::domain::runtime::fontconfig_for(&input.profile)),
                },
            )
            .await
            .with_context(|| format!("launching {}", input.binary.display()))?;

        let scraper = ChromiumScraper::from_browser(
            endpoint.browser,
            Some(build_stealth_config(&input.profile)),
        );
        let url = format!("file://{}", html_path.display());
        let raw = scraper.evaluate_on_url(&url, 0, READ_RESULTS).await;
        scraper.close().await;
        let _ = self.session.shutdown().await;
        let dom = raw.context("evaluating probes")?;
        let results = parse::extract(&dom, &probes)?;
        Ok(TestFingerprintOutput { probes: results })
    }
}
