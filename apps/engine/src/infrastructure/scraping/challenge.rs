use std::time::{Duration, Instant};

use chromiumoxide::Page;

use crate::domain::scraping::detection::is_challenge_page;

const POLL_INTERVAL: Duration = Duration::from_millis(750);

pub async fn wait_past_challenge(page: &Page, timeout: Duration) -> String {
    let deadline = Instant::now() + timeout;
    loop {
        let html = match page.content().await {
            Ok(h) => h,
            Err(_) => return String::new(),
        };
        if !is_challenge_page(&html) {
            return html;
        }
        if Instant::now() >= deadline {
            tracing::warn!("challenge did not resolve before timeout");
            return html;
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}
