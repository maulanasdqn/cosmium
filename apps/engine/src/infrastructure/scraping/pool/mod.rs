mod browser;
mod limits;

use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::Duration;

use anyhow::{Context, Result};

pub use browser::{Lease, PooledBrowser};
pub use limits::PoolLimits;
use limits::{Verdict, longest_idle};

const REAP_INTERVAL: Duration = Duration::from_secs(30);

type Slot = Arc<tokio::sync::Mutex<Option<Arc<PooledBrowser>>>>;
type SlotGuard = tokio::sync::OwnedMutexGuard<Option<Arc<PooledBrowser>>>;

pub struct BrowserPool {
    slots: Mutex<HashMap<String, Slot>>,
    limits: PoolLimits,
}

impl BrowserPool {
    pub fn start(limits: PoolLimits) -> Arc<Self> {
        let pool = Arc::new(Self {
            slots: Mutex::new(HashMap::new()),
            limits,
        });
        tokio::spawn(reap(Arc::downgrade(&pool)));
        pool
    }

    pub async fn acquire<F, Fut>(&self, key: &str, config_id: u64, launch: F) -> Result<Lease>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<PooledBrowser>> + Send,
    {
        let slot = self.slot(key);
        let mut current = slot.lock().await;
        if let Some(browser) = current.take() {
            let reused = self.reuse(&browser, config_id).await;
            match reused {
                Ok(lease) => {
                    *current = Some(browser);
                    return Ok(lease);
                }
                Err(verdict) => browser.retire(verdict.reason()).await,
            }
        }
        self.make_room(key).await;
        let browser = Arc::new(launch().await?);
        let lease = browser
            .lease()
            .context("freshly launched browser could not be leased")?;
        *current = Some(browser);
        Ok(lease)
    }

    pub async fn shutdown(&self) {
        for slot in self.slots_except(None) {
            retire_held(slot.lock_owned().await, "pool shutting down").await;
        }
    }

    async fn reuse(&self, browser: &Arc<PooledBrowser>, config_id: u64) -> Result<Lease, Verdict> {
        let verdict = self.limits.verdict(browser.usage(config_id));
        if verdict != Verdict::Keep {
            return Err(verdict);
        }
        let lease = browser.lease().ok_or(Verdict::Unhealthy)?;
        if lease.responsive().await {
            Ok(lease)
        } else {
            Err(Verdict::Unhealthy)
        }
    }

    async fn make_room(&self, key: &str) {
        let mut live = 0_usize;
        let mut idle = Vec::new();
        for slot in self.slots_except(Some(key)) {
            let Ok(guard) = slot.try_lock_owned() else {
                live = live.saturating_add(1);
                continue;
            };
            let Some(browser) = guard.as_ref() else {
                continue;
            };
            live = live.saturating_add(1);
            if !browser.in_use() {
                idle.push((browser.idle_for(), guard));
            }
        }
        let excess = self.limits.excess(live);
        if excess == 0 {
            return;
        }
        if idle.len() < excess {
            tracing::warn!(
                live,
                max = self.limits.max_browsers,
                "browser pool over capacity with every browser busy"
            );
        }
        for guard in longest_idle(idle, excess) {
            retire_held(guard, "evicted to make room").await;
        }
    }

    async fn evict_idle(&self) {
        for slot in self.slots_except(None) {
            let Ok(guard) = slot.try_lock_owned() else {
                continue;
            };
            let Some(browser) = guard.as_ref() else {
                continue;
            };
            if browser.in_use() {
                continue;
            }
            let verdict = self.limits.verdict(browser.usage(browser.config_id()));
            let reason = if verdict != Verdict::Keep {
                verdict.reason()
            } else if browser.idle_for() >= self.limits.idle_ttl {
                "idle"
            } else {
                continue;
            };
            retire_held(guard, reason).await;
        }
    }

    fn slot(&self, key: &str) -> Slot {
        let mut slots = self.slots.lock().unwrap_or_else(PoisonError::into_inner);
        Arc::clone(slots.entry(key.to_owned()).or_default())
    }

    fn slots_except(&self, key: Option<&str>) -> Vec<Slot> {
        let slots = self.slots.lock().unwrap_or_else(PoisonError::into_inner);
        slots
            .iter()
            .filter(|(k, _)| key != Some(k.as_str()))
            .map(|(_, slot)| Arc::clone(slot))
            .collect()
    }
}

async fn retire_held(mut guard: SlotGuard, reason: &str) {
    if let Some(browser) = guard.take() {
        browser.retire(reason).await;
    }
    drop(guard);
}

async fn reap(pool: Weak<BrowserPool>) {
    let mut ticker = tokio::time::interval(REAP_INTERVAL);
    ticker.tick().await;
    loop {
        ticker.tick().await;
        let Some(pool) = pool.upgrade() else {
            break;
        };
        pool.evict_idle().await;
    }
}
