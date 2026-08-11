mod drive;
mod eval;

use async_trait::async_trait;
use chromiumoxide::Page;
use chromiumoxide::browser::Browser;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, CaptureScreenshotFormat,
};
use chromiumoxide::page::ScreenshotParams;
use std::time::Duration;

use crate::domain::scraping::error::{ScrapeError, ScrapeResult};
use crate::domain::scraping::page::ScrapedPage;
use crate::domain::scraping::port::PageScraper;
use crate::domain::scraping::request::ScrapeRequest;

use super::stealth;
use super::stealth::StealthConfig;

const DEFAULT_NAV_TIMEOUT_SECS: u64 = 30;
const SCREENSHOT_QUALITY: i64 = 80;

pub struct ChromiumScraper {
    pub(crate) browser: Browser,
    pub(crate) stealth_config: Option<StealthConfig>,
}

impl ChromiumScraper {
    pub fn from_browser(browser: Browser, stealth_config: Option<StealthConfig>) -> Self {
        Self {
            browser,
            stealth_config,
        }
    }

    pub(crate) async fn new_stealth_page(&self) -> ScrapeResult<Page> {
        let page = self
            .browser
            .new_page("about:blank")
            .await
            .map_err(|e| ScrapeError::Connection(e.to_string()))?;
        let cmd = AddScriptToEvaluateOnNewDocumentParams::new(stealth::STEALTH_SCRIPT);
        let _ = page.execute(cmd).await;
        let net_cmd = AddScriptToEvaluateOnNewDocumentParams::new(stealth::STEALTH_NETWORK_SCRIPT);
        let _ = page.execute(net_cmd).await;
        if let Some(cfg) = &self.stealth_config {
            let _ = page.execute(cfg.cdp_ua_override()).await;
            let scripts = [
                cfg.navigator_overrides_script(),
                cfg.ua_data_script(),
                cfg.screen_script(),
                cfg.client_rects_script(),
                cfg.intl_script(),
                cfg.browser_state_script(),
            ];
            for s in scripts {
                let _ = page
                    .execute(AddScriptToEvaluateOnNewDocumentParams::new(s))
                    .await;
            }
        }
        Ok(page)
    }

    pub(crate) async fn navigate_tolerant(page: &Page, url: &str, wait_ms: u64) {
        let timeout = Duration::from_secs(DEFAULT_NAV_TIMEOUT_SECS);
        match tokio::time::timeout(timeout, page.goto(url)).await {
            Ok(Ok(_)) | Ok(Err(_)) | Err(_) => {}
        }
        if wait_ms > 0 {
            tokio::time::sleep(Duration::from_millis(wait_ms)).await;
        }
    }
}

#[async_trait]
impl PageScraper for ChromiumScraper {
    async fn scrape(&self, request: ScrapeRequest) -> ScrapeResult<ScrapedPage> {
        self.drive(&request).await
    }
}

pub(crate) async fn capture_screenshot(page: &Page) -> Vec<u8> {
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
