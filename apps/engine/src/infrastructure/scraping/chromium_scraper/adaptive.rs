use std::path::{Path, PathBuf};
use std::time::Duration;

use chromiumoxide::Page;

use crate::domain::scraping::detection::is_blocked;
use crate::domain::scraping::request::ScrapeRequest;

use super::super::challenge::wait_past_challenge;
use super::super::settle::{SettleWindow, wait_for_stable_content};
use super::super::status::StatusWatcher;
use super::ChromiumScraper;

const SETTLE_FLOOR_MS: u64 = 500;
const SETTLE_TIMEOUT_MS: u64 = 8000;
const MARKER_DIR: &str = "protected-hosts";

pub(super) struct LoadedTarget {
    pub(super) html: String,
    pub(super) final_url: String,
    pub(super) http_status: i32,
}

impl LoadedTarget {
    pub(super) fn blocked(&self) -> bool {
        is_blocked(self.http_status, self.html.as_bytes(), &self.final_url)
    }
}

pub(super) async fn load_target(page: &Page, request: &ScrapeRequest) -> LoadedTarget {
    let watcher = StatusWatcher::attach(page).await;
    ChromiumScraper::navigate_tolerant(page, &request.url, u64::from(request.wait_ms)).await;
    let nav_timeout = Duration::from_secs(super::DEFAULT_NAV_TIMEOUT_SECS);
    let past_challenge = wait_past_challenge(page, nav_timeout).await;
    let window = SettleWindow {
        floor: Duration::from_millis(SETTLE_FLOOR_MS),
        timeout: Duration::from_millis(SETTLE_TIMEOUT_MS),
    };
    let html = wait_for_stable_content(page, past_challenge, window).await;
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
    LoadedTarget {
        html,
        final_url,
        http_status,
    }
}

fn marker(dir: &Path, host: &str) -> PathBuf {
    let safe: String = host
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    dir.join(MARKER_DIR).join(safe)
}

pub(super) fn is_protected(dir: &Path, host: &str) -> bool {
    marker(dir, host).exists()
}

pub(super) async fn mark_protected(dir: &Path, host: &str) {
    let path = marker(dir, host);
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    let _ = tokio::fs::write(&path, b"").await;
}
