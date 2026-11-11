use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::{OwnedRwLockReadGuard, RwLock};

use crate::domain::scraping::port::BrowserSession;

use super::super::{ChromiumScraper, ProxyForwarder};
use super::limits::Usage;

const DRAIN_TIMEOUT: Duration = Duration::from_secs(300);
const FORCED_DRAIN_TIMEOUT: Duration = Duration::from_secs(30);
const PING_TIMEOUT: Duration = Duration::from_secs(3);

pub struct PooledBrowser {
    scraper: Arc<RwLock<Option<ChromiumScraper>>>,
    session: Arc<dyn BrowserSession>,
    _forwarder: Option<ProxyForwarder>,
    config_id: u64,
    born: Instant,
    served: AtomicU32,
    last_used_ms: AtomicU64,
    healthy: AtomicBool,
}

pub struct Lease {
    scraper: OwnedRwLockReadGuard<Option<ChromiumScraper>, ChromiumScraper>,
    browser: Arc<PooledBrowser>,
}

impl PooledBrowser {
    pub fn new(
        scraper: ChromiumScraper,
        session: Arc<dyn BrowserSession>,
        forwarder: Option<ProxyForwarder>,
        config_id: u64,
    ) -> Self {
        Self {
            scraper: Arc::new(RwLock::new(Some(scraper))),
            session,
            _forwarder: forwarder,
            config_id,
            born: Instant::now(),
            served: AtomicU32::new(0),
            last_used_ms: AtomicU64::new(0),
            healthy: AtomicBool::new(true),
        }
    }

    pub(super) const fn config_id(&self) -> u64 {
        self.config_id
    }

    pub(super) fn usage(&self, config_id: u64) -> Usage {
        Usage {
            healthy: self.healthy.load(Ordering::Relaxed),
            same_config: self.config_id == config_id,
            served: self.served.load(Ordering::Relaxed),
            age: self.born.elapsed(),
        }
    }

    pub(super) fn idle_for(&self) -> Duration {
        let last_used = Duration::from_millis(self.last_used_ms.load(Ordering::Relaxed));
        self.born.elapsed().saturating_sub(last_used)
    }

    pub(super) fn in_use(&self) -> bool {
        self.scraper.try_write().is_err()
    }

    pub(super) fn lease(self: &Arc<Self>) -> Option<Lease> {
        let guard = Arc::clone(&self.scraper).try_read_owned().ok()?;
        let scraper = OwnedRwLockReadGuard::try_map(guard, Option::as_ref).ok()?;
        Some(Lease {
            scraper,
            browser: Arc::clone(self),
        })
    }

    pub(super) async fn retire(&self, reason: &str) {
        tracing::info!(
            reason,
            served = self.served.load(Ordering::Relaxed),
            age_secs = self.born.elapsed().as_secs(),
            "retiring pooled browser"
        );
        let taken = self.take_exclusive().await;
        if let Some(scraper) = taken {
            scraper.close().await;
        } else {
            tracing::error!("pooled browser never drained; it dies when dropped");
        }
        if let Err(e) = self.session.shutdown().await {
            tracing::debug!(error = %e, "session shutdown failed");
        }
    }

    async fn take_exclusive(&self) -> Option<ChromiumScraper> {
        if let Some(scraper) = self.drain(DRAIN_TIMEOUT).await {
            return Some(scraper);
        }
        tracing::warn!(
            "in-flight scrapes outlived the drain timeout, closing the browser under them"
        );
        self.force_close().await;
        self.drain(FORCED_DRAIN_TIMEOUT).await
    }

    async fn drain(&self, limit: Duration) -> Option<ChromiumScraper> {
        tokio::time::timeout(limit, self.scraper.write())
            .await
            .ok()?
            .take()
    }

    async fn force_close(&self) {
        let Ok(guard) = self.scraper.try_read() else {
            return;
        };
        if let Some(scraper) = guard.as_ref() {
            scraper.request_close().await;
        }
        drop(guard);
    }

    fn touch(&self) {
        let now = u64::try_from(self.born.elapsed().as_millis()).unwrap_or(u64::MAX);
        self.last_used_ms.store(now, Ordering::Relaxed);
    }
}

impl Lease {
    pub fn scraper(&self) -> &ChromiumScraper {
        &self.scraper
    }

    pub fn mark_unhealthy(&self) {
        self.browser.healthy.store(false, Ordering::Relaxed);
    }

    pub(super) async fn responsive(&self) -> bool {
        let ping = self.scraper.browser.version();
        matches!(tokio::time::timeout(PING_TIMEOUT, ping).await, Ok(Ok(_)))
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        self.browser.served.fetch_add(1, Ordering::Relaxed);
        self.browser.touch();
    }
}
