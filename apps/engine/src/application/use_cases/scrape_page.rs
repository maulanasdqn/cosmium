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
use crate::infrastructure::scraping::ProxyForwarder;
use crate::infrastructure::scraping::ProxyPool;

use super::stealth_config::build_stealth_config;

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
    pub proxy_pool: Option<Arc<ProxyPool>>,
    pub headful: bool,
    pub wait_for_api: Option<String>,
}

pub struct ScrapePageOutput {
    pub page: ScrapedPage,
    pub blocked: bool,
    pub proxy_used: Option<String>,
}

impl ScrapePage {
    pub fn new(profile_repo: Arc<dyn ProfileRepository>, session: Arc<dyn BrowserSession>) -> Self {
        Self {
            profile_repo,
            session,
        }
    }

    pub async fn execute(&self, input: ScrapePageInput) -> Result<ScrapePageOutput> {
        let proxy = resolve_proxy(&input);
        self.execute_with_proxy(input, proxy).await
    }

    async fn execute_with_proxy(
        &self,
        input: ScrapePageInput,
        proxy: Option<ProxyConfig>,
    ) -> Result<ScrapePageOutput> {
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

        let proxy_url = proxy.as_ref().map(|p| p.url.clone());

        let _forwarder = if let Some(ref p) = proxy {
            match ProxyForwarder::start(p).await {
                Some(fwd) => {
                    flags.push(fwd.chrome_flag());
                    Some(fwd)
                }
                None => {
                    flags.push(format!("--proxy-server={}", p.url));
                    None
                }
            }
        } else {
            None
        };

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

        let stealth_config = build_stealth_config(&profile);
        let scraper = ChromiumScraper::from_browser(endpoint.browser, Some(stealth_config));

        let request = ScrapeRequest {
            url: input.url,
            wait_ms: input.wait_ms,
            screenshot: input.screenshot,
            workflow: input.workflow,
            proxy: proxy.clone(),
            wait_for_api: input.wait_for_api,
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

        if let (Some(pool), Some(url)) = (&input.proxy_pool, &proxy_url) {
            if blocked {
                pool.mark_failed(url);
            } else {
                pool.mark_success(url);
            }
        }

        let _ = self.session.shutdown().await;

        Ok(ScrapePageOutput {
            page,
            blocked,
            proxy_used: proxy_url,
        })
    }
}

fn resolve_proxy(input: &ScrapePageInput) -> Option<ProxyConfig> {
    if let Some(ref pool) = input.proxy_pool {
        pool.next_proxy()
    } else {
        input.proxy.clone()
    }
}
