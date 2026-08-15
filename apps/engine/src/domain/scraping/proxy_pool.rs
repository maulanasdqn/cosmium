use std::fmt;

use serde::{Deserialize, Serialize};

use super::request::ProxyConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RotationStrategy {
    RoundRobin,
    Random,
}

impl Default for RotationStrategy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

impl fmt::Display for RotationStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RoundRobin => write!(f, "round_robin"),
            Self::Random => write!(f, "random"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyHealth {
    Healthy,
    Failed { consecutive_failures: u32 },
    Cooldown,
}

#[derive(Debug, Clone)]
pub struct ProxyEntry {
    pub config: ProxyConfig,
    pub health: ProxyHealth,
    pub total_uses: u64,
    pub total_failures: u64,
    pub last_failure_epoch: Option<u64>,
}

impl ProxyEntry {
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            config,
            health: ProxyHealth::Healthy,
            total_uses: 0,
            total_failures: 0,
            last_failure_epoch: None,
        }
    }

    pub fn is_available(&self, now_epoch: u64, cooldown_secs: u64) -> bool {
        match self.health {
            ProxyHealth::Healthy => true,
            ProxyHealth::Failed { .. } | ProxyHealth::Cooldown => match self.last_failure_epoch {
                Some(ts) => now_epoch.saturating_sub(ts) >= cooldown_secs,
                None => true,
            },
        }
    }

    pub fn record_use(&mut self) {
        self.total_uses += 1;
    }

    pub fn record_failure(&mut self, now_epoch: u64) {
        self.total_failures += 1;
        self.last_failure_epoch = Some(now_epoch);
        match self.health {
            ProxyHealth::Healthy => {
                self.health = ProxyHealth::Failed {
                    consecutive_failures: 1,
                };
            }
            ProxyHealth::Failed {
                consecutive_failures,
            } => {
                self.health = ProxyHealth::Failed {
                    consecutive_failures: consecutive_failures + 1,
                };
            }
            ProxyHealth::Cooldown => {
                self.health = ProxyHealth::Failed {
                    consecutive_failures: 1,
                };
            }
        }
    }

    pub fn record_success(&mut self) {
        self.health = ProxyHealth::Healthy;
    }
}

#[derive(Debug, Clone)]
pub struct ProxyPoolConfig {
    pub strategy: RotationStrategy,
    pub cooldown_secs: u64,
    pub max_failures: u32,
}

impl Default for ProxyPoolConfig {
    fn default() -> Self {
        Self {
            strategy: RotationStrategy::RoundRobin,
            cooldown_secs: 60,
            max_failures: 3,
        }
    }
}

pub fn parse_proxy_list(input: &str) -> Vec<ProxyConfig> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| ProxyConfig {
            url: l.to_owned(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_proxy_list_filters_comments_and_blanks() {
        let input = "http://a:b@host1:8080\n\n# comment\nhttp://host2:3128\n  \n";
        let list = parse_proxy_list(input);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].url, "http://a:b@host1:8080");
        assert_eq!(list[1].url, "http://host2:3128");
    }

    #[test]
    fn proxy_entry_cooldown_logic() {
        let mut entry = ProxyEntry::new(ProxyConfig {
            url: "http://p:1".into(),
        });
        assert!(entry.is_available(100, 60));

        entry.record_failure(100);
        assert!(!entry.is_available(110, 60));
        assert!(entry.is_available(161, 60));

        entry.record_success();
        assert_eq!(entry.health, ProxyHealth::Healthy);
    }

    #[test]
    fn consecutive_failures_track() {
        let mut entry = ProxyEntry::new(ProxyConfig {
            url: "http://p:2".into(),
        });
        entry.record_failure(10);
        entry.record_failure(20);
        entry.record_failure(30);
        match entry.health {
            ProxyHealth::Failed {
                consecutive_failures,
            } => assert_eq!(consecutive_failures, 3),
            _ => panic!("expected Failed"),
        }
        assert_eq!(entry.total_failures, 3);
    }
}
