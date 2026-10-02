use std::time::Duration;

use chromiumoxide::Page;

use crate::domain::scraping::error::ScrapeResult;
use crate::domain::scraping::page::ScrapedPage;
use crate::domain::scraping::request::ScrapeRequest;

use super::super::cookies;
use super::super::datadome;
use super::super::fetch;
use super::super::network::ApiCapture;
use super::super::warmup;
use super::super::workflow;
use super::{ChromiumScraper, adaptive, capture_screenshot};

const API_CAPTURE_TIMEOUT_SECS: u64 = 30;

impl ChromiumScraper {
    pub(crate) async fn drive(&self, request: &ScrapeRequest) -> ScrapeResult<ScrapedPage> {
        let page = self.new_stealth_page().await?;

        let session_dir = crate::domain::runtime::session_cache_dir();
        let host = datadome::url_host(&request.url);
        if let Some(h) = host.as_deref() {
            datadome::preseed_cookies(&page, h, &session_dir).await;
        }

        let api_capture = match &request.wait_for_api {
            Some(pattern) => {
                tracing::info!("enabling API capture before warmup");
                ApiCapture::start(&page, pattern).await
            }
            None => None,
        };

        let known_protected = host
            .as_deref()
            .is_some_and(|h| adaptive::is_protected(&session_dir, h));

        if known_protected {
            tracing::info!("host previously challenged, warming up first");
            warmup::warmup_homepage(&page, &request.url).await;
            if request.wait_for_api.is_none() {
                if let Some(scraped) = fetch_bypass(&page, request).await {
                    return Ok(scraped);
                }
            }
        }

        let mut loaded = adaptive::load_target(&page, request).await;
        if !known_protected && loaded.blocked() {
            tracing::info!("target looks protected, escalating to warmup");
            if let Some(h) = host.as_deref() {
                adaptive::mark_protected(&session_dir, h).await;
            }
            warmup::warmup_homepage(&page, &request.url).await;
            loaded = adaptive::load_target(&page, request).await;
        }
        let mut html = loaded.html;

        let mut script_results = workflow::run(&page, &request.workflow).await;
        if !request.workflow.is_empty() {
            if let Ok(content) = page.content().await {
                html = content;
            }
        }

        if let Some(capture) = api_capture {
            let api_timeout = Duration::from_secs(API_CAPTURE_TIMEOUT_SECS);
            let responses = capture.wait_and_collect(&page, api_timeout).await;
            for (url, body) in responses {
                script_results.insert(format!("api:{url}"), body);
            }
        }

        let final_url = loaded.final_url;
        let http_status = loaded.http_status;
        let screenshot = maybe_screenshot(&page, request).await;
        let page_cookies = cookies::collect(&page).await;
        let user_agent = cookies::user_agent(&page).await;

        if let Some(h) = host.as_deref() {
            datadome::save_cookies(&page, h, &session_dir).await;
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

async fn fetch_bypass(page: &Page, request: &ScrapeRequest) -> Option<ScrapedPage> {
    let fetched_html = fetch::in_page_fetch(page, &request.url).await?;
    tracing::info!("in-page fetch bypass succeeded");
    let screenshot = maybe_screenshot(page, request).await;
    let page_cookies = cookies::collect(page).await;
    let user_agent = cookies::user_agent(page).await;
    let script_results = workflow::run(page, &request.workflow).await;
    Some(ScrapedPage {
        http_status: 200,
        html: fetched_html.into_bytes(),
        screenshot,
        final_url: request.url.clone(),
        cookies: page_cookies,
        user_agent,
        script_results,
    })
}

async fn maybe_screenshot(page: &Page, request: &ScrapeRequest) -> Vec<u8> {
    if request.screenshot {
        capture_screenshot(page).await
    } else {
        Vec::new()
    }
}
