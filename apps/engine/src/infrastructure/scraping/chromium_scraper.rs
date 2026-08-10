use std::time::Duration;

use async_trait::async_trait;
use chromiumoxide::browser::Browser;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, CaptureScreenshotFormat,
};
use chromiumoxide::page::ScreenshotParams;
use futures_util::StreamExt;

use crate::domain::scraping::error::{ScrapeError, ScrapeResult};
use crate::domain::scraping::page::ScrapedPage;
use crate::domain::scraping::port::PageScraper;
use crate::domain::scraping::request::ScrapeRequest;

use super::challenge::wait_past_challenge;
use super::cookies;
use super::settle::{SettleWindow, wait_for_stable_content};
use super::status::StatusWatcher;
use super::stealth;
use super::workflow;

const DEFAULT_NAV_TIMEOUT_SECS: u64 = 30;
const SETTLE_FLOOR_MS: u64 = 500;
const SETTLE_TIMEOUT_MS: u64 = 8000;
const SCREENSHOT_QUALITY: i64 = 80;

pub struct ChromiumScraper {
    browser: Browser,
    _handler_task: tokio::task::JoinHandle<()>,
}

impl ChromiumScraper {
    pub async fn connect(ws_url: &str) -> ScrapeResult<Self> {
        let (browser, mut handler) = Browser::connect(ws_url)
            .await
            .map_err(|e| ScrapeError::Connection(e.to_string()))?;

        let handler_task = tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                if event.is_err() {
                    break;
                }
            }
        });

        Ok(Self {
            browser,
            _handler_task: handler_task,
        })
    }

    async fn drive(&self, request: &ScrapeRequest) -> ScrapeResult<ScrapedPage> {
        let page = self
            .browser
            .new_page("about:blank")
            .await
            .map_err(|e| ScrapeError::Connection(e.to_string()))?;

        let stealth_cmd = AddScriptToEvaluateOnNewDocumentParams::new(stealth::STEALTH_SCRIPT);
        if let Err(e) = page.execute(stealth_cmd).await {
            tracing::warn!(error = %e, "stealth script injection failed");
        }

        let watcher = StatusWatcher::attach(&page).await;

        let nav_timeout = Duration::from_secs(DEFAULT_NAV_TIMEOUT_SECS);

        tokio::time::timeout(nav_timeout, page.goto(&request.url))
            .await
            .map_err(|_| ScrapeError::NavigationTimeout(nav_timeout.as_millis() as u64))?
            .map_err(|e| ScrapeError::Connection(e.to_string()))?;

        if request.wait_ms > 0 {
            tokio::time::sleep(Duration::from_millis(u64::from(request.wait_ms))).await;
        }

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

#[async_trait]
impl PageScraper for ChromiumScraper {
    async fn scrape(&self, request: ScrapeRequest) -> ScrapeResult<ScrapedPage> {
        self.drive(&request).await
    }
}

async fn capture_screenshot(page: &chromiumoxide::Page) -> Vec<u8> {
    let params = ScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Jpeg)
        .quality(SCREENSHOT_QUALITY)
        .full_page(false)
        .build();
    match page.screenshot(params).await {
        Ok(bytes) => bytes,
        Err(err) => {
            tracing::warn!(error = %err, "screenshot capture failed");
            Vec::new()
        }
    }
}
