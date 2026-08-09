use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};

use crate::domain::profile::ProfileRepository;
use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::{profile_to_env, profile_to_flags, user_data_dir};
use crate::domain::scraping::ScrapedPage;
use crate::domain::scraping::port::{BrowserSession, PageScraper};
use crate::domain::scraping::request::{ProxyConfig, ScrapeRequest};
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::scraping::ChromiumScraper;

pub struct ScrapePage {
    profile_repo: Arc<dyn ProfileRepository>,
    session: Arc<dyn BrowserSession>,
}

pub struct ScrapePageInput {
    pub profile: PathBuf,
    pub binary: PathBuf,
    pub url: String,
    pub wait_ms: u32,
    pub screenshot: bool,
    pub workflow: Vec<WorkflowStep>,
    pub proxy: Option<ProxyConfig>,
    pub headful: bool,
}

pub struct ScrapePageOutput {
    pub page: ScrapedPage,
    pub blocked: bool,
}

impl ScrapePage {
    pub fn new(profile_repo: Arc<dyn ProfileRepository>, session: Arc<dyn BrowserSession>) -> Self {
        Self {
            profile_repo,
            session,
        }
    }

    pub async fn execute(&self, input: ScrapePageInput) -> Result<ScrapePageOutput> {
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

        let scraper = ChromiumScraper::connect(&endpoint.ws_url)
            .await
            .map_err(|e| anyhow::anyhow!("CDP connection failed: {e}"))?;

        let request = ScrapeRequest {
            url: input.url,
            wait_ms: input.wait_ms,
            screenshot: input.screenshot,
            workflow: input.workflow,
            proxy: input.proxy,
        };

        let page = scraper
            .scrape(request)
            .await
            .map_err(|e| anyhow::anyhow!("scrape failed: {e}"))?;

        let blocked = crate::domain::scraping::detection::is_blocked(
            page.http_status as i16,
            &page.html,
            &page.final_url,
        );

        let _ = self.session.shutdown().await;

        Ok(ScrapePageOutput { page, blocked })
    }
}
