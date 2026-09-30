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
