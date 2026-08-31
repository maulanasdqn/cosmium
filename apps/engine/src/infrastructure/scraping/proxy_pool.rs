use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
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
        self.entries.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn next_proxy(&self) -> Option<ProxyConfig> {
        let now = epoch_secs();
        let mut entries = self.entries.lock().unwrap();
        let count = entries.len();
        if count == 0 {
            return None;
        }

        match self.config.strategy {
            RotationStrategy::RoundRobin => self.pick_round_robin(&mut entries, count, now),
            RotationStrategy::Random => self.pick_random(&mut entries, count, now),
        }
    }

    pub fn mark_success(&self, url: &str) {
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.iter_mut().find(|e| e.config.url == url) {
            entry.record_success();
        }
    }

    pub fn mark_failed(&self, url: &str) {
        let now = epoch_secs();
        let mut entries = self.entries.lock().unwrap();
        if let Some(entry) = entries.iter_mut().find(|e| e.config.url == url) {
            entry.record_failure(now);
        }
    }

    pub fn available_count(&self) -> usize {
        let now = epoch_secs();
        let entries = self.entries.lock().unwrap();
        entries
            .iter()
            .filter(|e| e.is_available(now, self.config.cooldown_secs))
            .count()
    }

    pub fn stats(&self) -> Vec<ProxyStats> {
        let now = epoch_secs();
        let entries = self.entries.lock().unwrap();
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
            if entries[idx].is_available(now, self.config.cooldown_secs) {
                entries[idx].record_use();
                return Some(entries[idx].config.clone());
            }
        }
        entries[start % count].record_use();
        Some(entries[start % count].config.clone())
    }

    fn pick_random(
        &self,
        entries: &mut [ProxyEntry],
        count: usize,
        now: u64,
    ) -> Option<ProxyConfig> {
        let available: Vec<usize> = (0..count)
            .filter(|&i| entries[i].is_available(now, self.config.cooldown_secs))
            .collect();

        let idx = if available.is_empty() {
            cheap_random(count)
        } else {
            available[cheap_random(available.len())]
        };

        entries[idx].record_use();
        Some(entries[idx].config.clone())
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
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (mix as usize) % bound
}

#[derive(Debug, Clone)]
pub struct ProxyStats {
    pub url: String,
    pub available: bool,
    pub total_uses: u64,
    pub total_failures: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::scraping::proxy_pool::ProxyPoolConfig;

    fn make_proxies(n: usize) -> Vec<ProxyConfig> {
        (0..n)
            .map(|i| ProxyConfig {
                url: format!("http://proxy{i}:8080"),
            })
            .collect()
    }

    #[test]
    fn round_robin_cycles() {
        let pool = ProxyPool::new(make_proxies(3), ProxyPoolConfig::default());
        let a = pool.next_proxy().unwrap().url;
        let b = pool.next_proxy().unwrap().url;
        let c = pool.next_proxy().unwrap().url;
        let d = pool.next_proxy().unwrap().url;
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_eq!(a, d);
    }

    #[test]
    fn skips_failed_proxy() {
        let pool = ProxyPool::new(make_proxies(2), ProxyPoolConfig::default());
        pool.mark_failed("http://proxy0:8080");
        let picked = pool.next_proxy().unwrap().url;
        assert_eq!(picked, "http://proxy1:8080");
    }

    #[test]
    fn empty_pool_returns_none() {
        let pool = ProxyPool::new(vec![], ProxyPoolConfig::default());
        assert!(pool.next_proxy().is_none());
    }

    #[test]
    fn stats_reports_all() {
        let pool = ProxyPool::new(make_proxies(2), ProxyPoolConfig::default());
        pool.next_proxy();
        pool.mark_failed("http://proxy1:8080");
        let stats = pool.stats();
        assert_eq!(stats.len(), 2);
    }
}
