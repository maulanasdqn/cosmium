use std::time::Duration;

use crate::domain::scraping::error::{ScrapeError, ScrapeResult};

use super::super::warmup;
use super::ChromiumScraper;

impl ChromiumScraper {
    pub async fn evaluate_on_url(
        &self,
        url: &str,
        wait_ms: u64,
        script: &str,
    ) -> ScrapeResult<String> {
        let page = self.new_stealth_page().await?;
        warmup::warmup_homepage(&page, url).await;
        Self::navigate_tolerant(&page, url, wait_ms).await;
        let wrapper = format!(
            "(async () => {{ try {{ return String(await (async () => {{ {script} }})()) }} catch(e) {{ return 'JS_ERROR:' + e.message }} }})()"
        );
        let eval_timeout = Duration::from_secs(60);
        let result = tokio::time::timeout(eval_timeout, page.evaluate(wrapper))
            .await
            .map_err(|_| ScrapeError::WorkflowFailed("script timed out after 60s".into()))?
            .map_err(|e| ScrapeError::WorkflowFailed(e.to_string()))?;
        Ok(result
            .into_value::<String>()
            .unwrap_or_else(|_| "null".into()))
    }
}
