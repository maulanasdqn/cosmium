use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PoolLimits {
    pub max_browsers: usize,
    pub max_pages: u32,
    pub max_age: Duration,
    pub idle_ttl: Duration,
}

impl Default for PoolLimits {
    fn default() -> Self {
        Self {
            max_browsers: 4,
            max_pages: 50,
            max_age: Duration::from_secs(30 * 60),
            idle_ttl: Duration::from_secs(5 * 60),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Verdict {
    Keep,
    Unhealthy,
    Reconfigured,
    Exhausted,
    Expired,
}

impl Verdict {
    pub(super) const fn reason(self) -> &'static str {
        match self {
            Self::Keep => "kept",
            Self::Unhealthy => "unhealthy",
            Self::Reconfigured => "launch config changed",
            Self::Exhausted => "page budget spent",
            Self::Expired => "max age reached",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Usage {
    pub(super) healthy: bool,
    pub(super) same_config: bool,
    pub(super) served: u32,
    pub(super) age: Duration,
}

impl PoolLimits {
    pub(super) fn verdict(&self, usage: Usage) -> Verdict {
        if !usage.healthy {
            Verdict::Unhealthy
        } else if !usage.same_config {
            Verdict::Reconfigured
        } else if usage.served >= self.max_pages {
            Verdict::Exhausted
        } else if usage.age >= self.max_age {
            Verdict::Expired
        } else {
            Verdict::Keep
        }
    }

    pub(super) const fn excess(&self, live_others: usize) -> usize {
        live_others
            .saturating_add(1)
            .saturating_sub(self.max_browsers)
    }
}

pub(super) fn longest_idle<T>(mut idle: Vec<(Duration, T)>, count: usize) -> Vec<T> {
    idle.sort_by(|a, b| b.0.cmp(&a.0));
    idle.into_iter().take(count).map(|(_, item)| item).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn fresh() -> Usage {
        Usage {
            healthy: true,
            same_config: true,
            served: 0,
            age: Duration::ZERO,
        }
    }

    #[test]
    fn a_fresh_browser_is_kept() {
        assert_eq!(PoolLimits::default().verdict(fresh()), Verdict::Keep);
    }

    #[test]
    fn health_outranks_every_other_reason() {
        let usage = Usage {
            healthy: false,
            same_config: false,
            served: u32::MAX,
            age: Duration::MAX,
        };
        assert_eq!(PoolLimits::default().verdict(usage), Verdict::Unhealthy);
    }

    #[test]
    fn a_changed_launch_config_forces_a_relaunch() {
        let usage = Usage {
            same_config: false,
            ..fresh()
        };
        assert_eq!(PoolLimits::default().verdict(usage), Verdict::Reconfigured);
    }

    #[test]
    fn page_budget_and_age_both_recycle() {
        let limits = PoolLimits::default();
        let spent = Usage {
            served: limits.max_pages,
            ..fresh()
        };
        let old = Usage {
            age: limits.max_age,
            ..fresh()
        };
        assert_eq!(limits.verdict(spent), Verdict::Exhausted);
        assert_eq!(limits.verdict(old), Verdict::Expired);
    }

    #[test]
    fn excess_counts_the_browser_about_to_launch() {
        let limits = PoolLimits {
            max_browsers: 2,
            ..PoolLimits::default()
        };
        assert_eq!(limits.excess(0), 0);
        assert_eq!(limits.excess(1), 0);
        assert_eq!(limits.excess(2), 1);
        assert_eq!(limits.excess(5), 4);
    }

    #[test]
    fn longest_idle_evicts_the_stalest_first() {
        let idle = vec![
            (Duration::from_secs(5), "warm"),
            (Duration::from_secs(90), "stale"),
            (Duration::from_secs(30), "cooling"),
        ];
        assert_eq!(longest_idle(idle, 2), vec!["stale", "cooling"]);
    }

    #[test]
    fn longest_idle_returns_what_it_has_when_short() {
        let idle = vec![(Duration::from_secs(1), 1)];
        assert_eq!(longest_idle(idle, 3), vec![1]);
    }
}
