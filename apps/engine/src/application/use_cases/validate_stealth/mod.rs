pub mod targets;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};

use crate::domain::profile::ProfileRepository;
use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::{profile_to_env, profile_to_flags, user_data_dir};
use crate::domain::scraping::port::BrowserSession;
use crate::domain::scraping::validation::{ValidationResult, ValidationTarget};
use crate::infrastructure::scraping::ChromiumScraper;

pub struct ValidateStealth {
    profile_repo: Arc<dyn ProfileRepository>,
    session: Arc<dyn BrowserSession>,
}

pub struct ValidateStealthInput {
    pub profile: PathBuf,
    pub binary: PathBuf,
    pub targets: Vec<ValidationTarget>,
    pub headful: bool,
}

pub struct ValidateStealthOutput {
    pub results: Vec<ValidationResult>,
}

impl ValidateStealth {
    pub fn new(profile_repo: Arc<dyn ProfileRepository>, session: Arc<dyn BrowserSession>) -> Self {
        Self {
            profile_repo,
            session,
        }
    }

    pub async fn execute(&self, input: ValidateStealthInput) -> Result<ValidateStealthOutput> {
        let profile = self
            .profile_repo
            .load(&input.profile)
            .await
            .with_context(|| format!("loading {}", input.profile.display()))?;

        let mut flags = profile_to_flags(&profile);
        let env = profile_to_env(&profile);
        let data_dir = user_data_dir(&profile.name);

        if !input.headful {
            flags.push("--headless=new".into());
        }
        if !flags
            .iter()
            .any(|f| f.contains("cosmium-strip-automation-tells"))
        {
            flags.push("--cosmium-strip-automation-tells".into());
        }

        let endpoint = self
            .session
            .launch_with_cdp(
                Path::new(&input.binary),
                LaunchSpec {
                    flags,
                    urls: Vec::new(),
                    env,
                    user_data_dir: Some(data_dir),
                },
            )
            .await
            .with_context(|| format!("launching {}", input.binary.display()))?;

        let stealth_config =
            crate::application::use_cases::stealth_config::build_stealth_config(&profile);
        let scraper = ChromiumScraper::from_browser(endpoint.browser, Some(stealth_config));

        let mut results = Vec::with_capacity(input.targets.len());
        for target in &input.targets {
            let r = run_target(&scraper, target).await;
            results.push(r);
        }

        let _ = self.session.shutdown().await;
        Ok(ValidateStealthOutput { results })
    }
}

async fn run_target(scraper: &ChromiumScraper, target: &ValidationTarget) -> ValidationResult {
    let start = Instant::now();
    let raw = match scraper
        .evaluate_on_url(&target.url, target.wait_ms, &target.extractor)
        .await
    {
        Ok(v) => v,
        Err(e) => format!("ERROR: {e}"),
    };
    let duration_ms = start.elapsed().as_millis() as u64;
    let (verdict, detail) = targets::evaluate(&target.name, &raw);
    ValidationResult {
        target: target.name.clone(),
        verdict,
        detail,
        raw,
        duration_ms,
    }
}
