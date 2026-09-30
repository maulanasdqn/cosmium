use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::scraping::proxy_pool::{ProxyEntry, ProxyPoolConfig, RotationStrategy};
use crate::domain::scraping::request::ProxyConfig;

pub struct ProxyPool {
    entries: Mutex<Vec<ProxyEntry>>,
    config: ProxyPoolConfig,
    cursor: AtomicUsize,
}

impl ProxyPool {
    pub fn new(proxies: Vec<ProxyConfig>, config: ProxyPoolConfig) -> Self {
        let entries = proxies.into_iter().map(ProxyEntry::new).collect();
        Self {
            entries: Mutex::new(entries),
            config,
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn len(&self) -> usize {
        self.entries
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn next_proxy(&self) -> Option<ProxyConfig> {
        let now = epoch_secs();
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        let count = entries.len();
        if count == 0 {
            return None;
        }

        let picked = match self.config.strategy {
            RotationStrategy::RoundRobin => self.pick_round_robin(&mut entries, count, now),
            RotationStrategy::Random => self.pick_random(&mut entries, count, now),
        };
        drop(entries);
        picked
    }

    pub fn mark_success(&self, url: &str) {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(entry) = entries.iter_mut().find(|e| e.config.url == url) {
            entry.record_success();
        }
    }

    pub fn mark_failed(&self, url: &str) {
        let now = epoch_secs();
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(entry) = entries.iter_mut().find(|e| e.config.url == url) {
            entry.record_failure(now);
        }
    }

    pub fn available_count(&self) -> usize {
        let now = epoch_secs();
        let entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries
            .iter()
            .filter(|e| e.is_available(now, self.config.cooldown_secs))
            .count()
    }

    pub fn stats(&self) -> Vec<ProxyStats> {
        let now = epoch_secs();
        let entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries
            .iter()
            .map(|e| ProxyStats {
                url: e.config.url.clone(),
                available: e.is_available(now, self.config.cooldown_secs),
                total_uses: e.total_uses,
                total_failures: e.total_failures,
            })
            .collect()
    }

    fn pick_round_robin(
        &self,
        entries: &mut [ProxyEntry],
        count: usize,
        now: u64,
    ) -> Option<ProxyConfig> {
        let start = self.cursor.fetch_add(1, Ordering::Relaxed) % count;
        for i in 0..count {
            let idx = (start + i) % count;
            if let Some(entry) = entries.get_mut(idx) {
                if entry.is_available(now, self.config.cooldown_secs) {
                    entry.record_use();
                    return Some(entry.config.clone());
                }
            }
        }
        let entry = entries.get_mut(start)?;
        entry.record_use();
        Some(entry.config.clone())
    }

    fn pick_random(
        &self,
        entries: &mut [ProxyEntry],
        count: usize,
        now: u64,
    ) -> Option<ProxyConfig> {
        let available: Vec<usize> = (0..count)
            .filter(|&i| {
                entries
                    .get(i)
                    .is_some_and(|e| e.is_available(now, self.config.cooldown_secs))
            })
            .collect();

        let idx = if available.is_empty() {
            cheap_random(count)
        } else {
            *available.get(cheap_random(available.len()))?
        };

        let entry = entries.get_mut(idx)?;
        entry.record_use();
        Some(entry.config.clone())
    }
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn cheap_random(bound: usize) -> usize {
    let t = epoch_secs();
    let mix = t
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    usize::try_from(mix % bound as u64).unwrap_or_default()
}

#[derive(Debug, Clone)]
pub struct ProxyStats {
    pub url: String,
    pub available: bool,
    pub total_uses: u64,
    pub total_failures: u64,
}

#[cfg(test)]
#[path = "proxy_pool_tests.rs"]
mod tests;
