const MS_PER_DAY: u64 = 86_400_000;

#[derive(Debug, Clone, Copy)]
pub struct Retention {
    pub max_results: usize,
    pub max_age_days: u64,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            max_results: 200,
            max_age_days: 30,
        }
    }
}

impl Retention {
    pub const fn unlimited() -> Self {
        Self {
            max_results: 0,
            max_age_days: 0,
        }
    }

    const fn is_unlimited(self) -> bool {
        self.max_results == 0 && self.max_age_days == 0
    }
}

pub(super) fn expired_ids(
    entries: &[(String, u64)],
    policy: Retention,
    now_ms: u64,
) -> Vec<String> {
    if entries.is_empty() || policy.is_unlimited() {
        return Vec::new();
    }
    let mut newest_first: Vec<&(String, u64)> = entries.iter().collect();
    newest_first.sort_by_key(|(id, created)| (std::cmp::Reverse(*created), id.clone()));

    let cutoff = (policy.max_age_days > 0)
        .then(|| policy.max_age_days.saturating_mul(MS_PER_DAY))
        .map(|span| now_ms.saturating_sub(span));

    newest_first
        .into_iter()
        .enumerate()
        .filter(|(index, (_, created))| {
            let over_count = policy.max_results > 0 && *index >= policy.max_results;
            let too_old = cutoff.is_some_and(|limit| *created < limit);
            over_count || too_old
        })
        .map(|(_, (id, _))| id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Retention, expired_ids};

    const NOW: u64 = 100 * super::MS_PER_DAY;

    fn entry(id: &str, days_ago: u64) -> (String, u64) {
        (
            id.to_owned(),
            NOW.saturating_sub(days_ago.saturating_mul(super::MS_PER_DAY)),
        )
    }

    #[test]
    fn keeps_everything_when_unlimited() {
        let entries = vec![entry("a", 0), entry("b", 900)];
        assert!(expired_ids(&entries, Retention::unlimited(), NOW).is_empty());
    }

    #[test]
    fn drops_oldest_beyond_the_count_limit() {
        let entries = vec![entry("old", 3), entry("new", 1), entry("mid", 2)];
        let policy = Retention {
            max_results: 2,
            max_age_days: 0,
        };
        assert_eq!(expired_ids(&entries, policy, NOW), vec!["old".to_owned()]);
    }

    #[test]
    fn drops_entries_past_the_age_limit() {
        let entries = vec![entry("fresh", 1), entry("stale", 40)];
        let policy = Retention {
            max_results: 0,
            max_age_days: 30,
        };
        assert_eq!(expired_ids(&entries, policy, NOW), vec!["stale".to_owned()]);
    }

    #[test]
    fn applies_both_limits_without_duplicates() {
        let entries = vec![entry("a", 1), entry("b", 2), entry("c", 60), entry("d", 61)];
        let policy = Retention {
            max_results: 3,
            max_age_days: 30,
        };
        let mut expired = expired_ids(&entries, policy, NOW);
        expired.sort();
        assert_eq!(expired, vec!["c".to_owned(), "d".to_owned()]);
    }

    #[test]
    fn keeps_exactly_the_count_limit() {
        let entries = vec![entry("a", 1), entry("b", 2)];
        let policy = Retention {
            max_results: 2,
            max_age_days: 0,
        };
        assert!(expired_ids(&entries, policy, NOW).is_empty());
    }

    #[test]
    fn handles_empty_input() {
        assert!(expired_ids(&[], Retention::default(), NOW).is_empty());
    }

    #[test]
    fn breaks_ties_deterministically() {
        let entries = vec![entry("b", 5), entry("a", 5), entry("c", 5)];
        let policy = Retention {
            max_results: 1,
            max_age_days: 0,
        };
        let expired = expired_ids(&entries, policy, NOW);
        assert_eq!(expired, vec!["b".to_owned(), "c".to_owned()]);
    }
}
