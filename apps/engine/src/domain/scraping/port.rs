use std::path::Path;

use async_trait::async_trait;

use crate::domain::runtime::RuntimeResult;
use crate::domain::runtime::browser::LaunchSpec;

use super::error::ScrapeResult;
use super::page::ScrapedPage;
use super::request::ScrapeRequest;

pub struct CdpEndpoint {
    pub ws_url: String,
}

#[async_trait]
pub trait BrowserSession: Send + Sync {
    async fn launch_with_cdp(&self, binary: &Path, spec: LaunchSpec) -> RuntimeResult<CdpEndpoint>;

    async fn shutdown(&self) -> RuntimeResult<()>;
}

#[async_trait]
pub trait PageScraper: Send + Sync {
    async fn scrape(&self, request: ScrapeRequest) -> ScrapeResult<ScrapedPage>;
}
