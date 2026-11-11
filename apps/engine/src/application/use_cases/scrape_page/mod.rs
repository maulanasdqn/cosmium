mod launch;
mod retry;

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};

use crate::domain::profile::{Profile, ProfileRepository};
use crate::domain::scraping::ScrapedPage;
use crate::domain::scraping::detection::is_blocked;
use crate::domain::scraping::port::PageScraper;
use crate::domain::scraping::request::{ProxyConfig, ScrapeRequest};
use crate::domain::scraping::workflow::WorkflowStep;
use crate::infrastructure::scraping::{BrowserPool, ProxyPool};

pub struct ScrapePage {
    profile_repo: Arc<dyn ProfileRepository>,
    browsers: Arc<BrowserPool>,
}

#[derive(Clone)]
pub struct ScrapePageInput {
    pub profile: PathBuf,
    pub binary: PathBuf,
    pub url: String,
    pub wait_ms: u32,
    pub screenshot: bool,
    pub workflow: Vec<WorkflowStep>,
    pub proxy: Option<ProxyConfig>,
    pub proxy_pool: Option<Arc<ProxyPool>>,
    pub headful: bool,
    pub wait_for_api: Option<String>,
    pub geo_sync: bool,
}

pub struct ScrapePageOutput {
    pub page: ScrapedPage,
    pub blocked: bool,
    pub proxy_used: Option<String>,
}

impl ScrapePage {
    pub fn new(profile_repo: Arc<dyn ProfileRepository>, browsers: Arc<BrowserPool>) -> Self {
        Self {
            profile_repo,
            browsers,
        }
    }

    pub async fn execute(&self, input: &ScrapePageInput) -> Result<ScrapePageOutput> {
        let proxy = resolve_proxy(input);
        let proxy_url = proxy.as_ref().map(|p| p.url.clone());
        let profile = self.load_profile(input).await?;
        let key = launch::pool_key(&profile.name, proxy_url.as_deref());
        let config_id = launch::config_id(input, &profile);
        let launch_proxy = proxy.clone();
        let lease = self
            .browsers
            .acquire(&key, config_id, || {
                launch::launch(input, profile, launch_proxy, config_id)
            })
            .await?;

        let outcome = lease.scraper().scrape(request_for(input, proxy)).await;
        if outcome.is_err() {
            lease.mark_unhealthy();
        }
        drop(lease);
        let page = outcome.map_err(|e| anyhow::anyhow!("scrape failed: {e}"))?;

        let blocked = is_blocked(page.http_status, &page.html, &page.final_url);
        if let (Some(pool), Some(url)) = (&input.proxy_pool, &proxy_url) {
            if blocked {
                pool.mark_failed(url);
            } else {
                pool.mark_success(url);
            }
        }

        Ok(ScrapePageOutput {
            page,
            blocked,
            proxy_used: proxy_url,
        })
    }

    async fn load_profile(&self, input: &ScrapePageInput) -> Result<Profile> {
        self.profile_repo
            .load(&input.profile)
            .await
            .with_context(|| format!("loading {}", input.profile.display()))
    }
}

fn request_for(input: &ScrapePageInput, proxy: Option<ProxyConfig>) -> ScrapeRequest {
    ScrapeRequest {
        url: input.url.clone(),
        wait_ms: input.wait_ms,
        screenshot: input.screenshot,
        workflow: input.workflow.clone(),
        proxy,
        wait_for_api: input.wait_for_api.clone(),
    }
}

fn resolve_proxy(input: &ScrapePageInput) -> Option<ProxyConfig> {
    input
        .proxy_pool
        .as_ref()
        .map_or_else(|| input.proxy.clone(), |pool| pool.next_proxy())
}
