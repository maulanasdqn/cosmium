mod adaptive;
mod drive;
mod eval;
mod native;

use async_trait::async_trait;
use chromiumoxide::Page;
use chromiumoxide::browser::Browser;
use chromiumoxide::cdp::browser_protocol::page::{
    AddScriptToEvaluateOnNewDocumentParams, CaptureScreenshotFormat,
};
use chromiumoxide::page::ScreenshotParams;
use std::time::Duration;
use tokio::sync::OnceCell;

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
    native: OnceCell<bool>,
}

impl ChromiumScraper {
    pub const fn from_browser(browser: Browser, stealth_config: Option<StealthConfig>) -> Self {
        Self {
            browser,
            stealth_config,
            native: OnceCell::const_new(),
        }
    }

    pub(crate) async fn new_stealth_page(&self) -> ScrapeResult<Page> {
        let page = self
            .browser
            .new_page("about:blank")
            .await
            .map_err(|e| ScrapeError::Connection(e.to_string()))?;
        native::block_local_network(&page).await;
        native::emulate_focus(&page).await;
        let Some(cfg) = &self.stealth_config else {
            return Ok(page);
        };
        let native = *self
            .native
            .get_or_init(|| native::is_native_build(&page, cfg))
            .await;
        if native {
            let _ = page.execute(cfg.cdp_ua_override()).await;
        } else {
            inject_js_fallback(&page, cfg).await;
        }
        Ok(page)
    }

    pub(crate) async fn navigate_tolerant(page: &Page, url: &str, wait_ms: u64) {
        let timeout = Duration::from_secs(DEFAULT_NAV_TIMEOUT_SECS);
        match tokio::time::timeout(timeout, page.goto(url)).await {
            Ok(Ok(_) | Err(_)) | Err(_) => {}
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

async fn inject_js_fallback(page: &Page, cfg: &StealthConfig) {
    let _ = page.execute(cfg.cdp_ua_override()).await;
    let scripts = [
        stealth::STEALTH_SCRIPT.to_owned(),
        cfg.navigator_overrides_script(),
        cfg.ua_data_script(),
        cfg.screen_script(),
        cfg.client_rects_script(),
        cfg.intl_script(),
        cfg.browser_state_script(),
        cfg.webgl_script(),
        cfg.canvas_script(),
        cfg.audio_script(),
    ];
    for s in scripts {
        let _ = page
            .execute(AddScriptToEvaluateOnNewDocumentParams::new(s))
            .await;
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
