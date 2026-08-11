use std::time::Duration;

use crate::domain::scraping::error::ScrapeResult;
use crate::domain::scraping::page::ScrapedPage;
use crate::domain::scraping::request::ScrapeRequest;

use super::super::challenge::wait_past_challenge;
use super::super::cookies;
use super::super::datadome;
use super::super::fetch;
use super::super::settle::{SettleWindow, wait_for_stable_content};
use super::super::status::StatusWatcher;
use super::super::warmup;
use super::super::workflow;
use super::{ChromiumScraper, capture_screenshot};

const SETTLE_FLOOR_MS: u64 = 500;
const SETTLE_TIMEOUT_MS: u64 = 8000;

impl ChromiumScraper {
    pub(crate) async fn drive(&self, request: &ScrapeRequest) -> ScrapeResult<ScrapedPage> {
        let page = self.new_stealth_page().await?;

        let session_dir = crate::domain::runtime::session_cache_dir();
        if let Some(host) = datadome::url_host(&request.url) {
            datadome::preseed_cookies(&page, &host, &session_dir).await;
        }

        warmup::warmup_homepage(&page, &request.url).await;

        let home_html = warmup::cdp_content_timeout(&page, Duration::from_secs(10)).await;
        let _home_verdict = match &home_html {
            Some(html) => {
                let v = datadome::classify(html);
                match &v {
                    datadome::DdVerdict::Clean => {
                        tracing::info!("homepage clean, trying in-page fetch");
                    }
                    datadome::DdVerdict::SoftChallenge => {
                        tracing::info!("homepage has DataDome challenge, waiting for c.js");
                        let initial_cookie = datadome::get_dd_cookie_value(&page).await;
                        let resolved =
                            datadome::wait_for_challenge_js(&page, initial_cookie.as_deref()).await;
                        if matches!(resolved, datadome::DdVerdict::Clean) {
                            tracing::info!("DataDome challenge resolved");
                        }
                    }
                    datadome::DdVerdict::HardBlock => {
                        tracing::warn!("homepage hard-blocked by DataDome");
                    }
                }
                v
            }
            None => {
                tracing::info!("page.content() timed out, using cookie-based DataDome detection");
                let initial_cookie = datadome::get_dd_cookie_value(&page).await;
                datadome::wait_for_challenge_js(&page, initial_cookie.as_deref()).await
            }
        };

        if let Some(host) = datadome::url_host(&request.url) {
            datadome::save_cookies(&page, &host, &session_dir).await;
        }

        if let Some(fetched_html) = fetch::in_page_fetch(&page, &request.url).await {
            tracing::info!("in-page fetch bypass succeeded");
            let screenshot = if request.screenshot {
                capture_screenshot(&page).await
            } else {
                Vec::new()
            };
            let page_cookies = cookies::collect(&page).await;
            let user_agent = cookies::user_agent(&page).await;
            let script_results = workflow::run(&page, &request.workflow).await;
            return Ok(ScrapedPage {
                http_status: 200,
                html: fetched_html.into_bytes(),
                screenshot,
                final_url: request.url.clone(),
                cookies: page_cookies,
                user_agent,
                script_results,
            });
        }

        tracing::info!("in-page fetch unavailable, navigating directly");
        let watcher = StatusWatcher::attach(&page).await;
        Self::navigate_tolerant(&page, &request.url, u64::from(request.wait_ms)).await;

        let nav_timeout = Duration::from_secs(super::DEFAULT_NAV_TIMEOUT_SECS);
        let past_challenge = wait_past_challenge(&page, nav_timeout).await;
        let window = SettleWindow {
            floor: Duration::from_millis(SETTLE_FLOOR_MS),
            timeout: Duration::from_millis(SETTLE_TIMEOUT_MS),
        };
        let mut html = wait_for_stable_content(&page, past_challenge, window).await;

        let script_results = workflow::run(&page, &request.workflow).await;
        if !request.workflow.is_empty() {
            if let Ok(content) = page.content().await {
                html = content;
            }
        }

        let final_url = page
            .url()
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| request.url.clone());
        let http_status = match &watcher {
            Some(w) => w.status_for(&final_url).await,
            None => 200,
        };
        let screenshot = if request.screenshot {
            capture_screenshot(&page).await
        } else {
            Vec::new()
        };
        let page_cookies = cookies::collect(&page).await;
        let user_agent = cookies::user_agent(&page).await;

        if let Some(host) = datadome::url_host(&request.url) {
            datadome::save_cookies(&page, &host, &session_dir).await;
        }

        Ok(ScrapedPage {
            http_status,
            html: html.into_bytes(),
            screenshot,
            final_url,
            cookies: page_cookies,
            user_agent,
            script_results,
        })
    }
}
