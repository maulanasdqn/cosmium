use std::time::Duration;

use anyhow::{Result, anyhow, bail};
use rand::Rng;

use super::{ScrapePage, ScrapePageInput, ScrapePageOutput};

const BASE_BACKOFF_MS: u64 = 1_000;
const MAX_BACKOFF_SHIFT: u32 = 3;
const MAX_JITTER_MS: u64 = 500;

impl ScrapePage {
    pub async fn execute_with_retries(
        &self,
        input: &ScrapePageInput,
        retries: u32,
    ) -> Result<(ScrapePageOutput, u32)> {
        if !input.binary.exists() {
            bail!("browser binary not found at {}", input.binary.display());
        }
        self.load_profile(input).await?;

        let max_attempts = retries.saturating_add(1);
        let mut blocked = None;
        let mut failure = None;
        for attempt in 1..=max_attempts {
            if attempt > 1 {
                let delay = backoff(attempt, rand::rng().random_range(0..=MAX_JITTER_MS));
                tracing::info!(
                    attempt,
                    max_attempts,
                    delay_ms = delay.as_millis(),
                    "retrying scrape"
                );
                tokio::time::sleep(delay).await;
            }
            match self.execute(input).await {
                Ok(out) if !out.blocked => return Ok((out, attempt)),
                Ok(out) => {
                    let proxy = out.proxy_used.as_deref().unwrap_or("direct");
                    tracing::warn!(attempt, proxy, "page blocked");
                    blocked = Some(out);
                }
                Err(e) => {
                    tracing::warn!(attempt, error = format!("{e:#}"), "scrape attempt failed");
                    failure = Some(e);
                }
            }
        }

        blocked
            .map(|out| (out, max_attempts))
            .ok_or_else(|| failure.unwrap_or_else(|| anyhow!("no scrape attempt was made")))
    }
}

fn backoff(attempt: u32, jitter_ms: u64) -> Duration {
    let shift = attempt.saturating_sub(2).min(MAX_BACKOFF_SHIFT);
    Duration::from_millis((BASE_BACKOFF_MS << shift).saturating_add(jitter_ms.min(MAX_JITTER_MS)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_from_one_second() {
        assert_eq!(backoff(2, 0), Duration::from_secs(1));
        assert_eq!(backoff(3, 0), Duration::from_secs(2));
        assert_eq!(backoff(4, 0), Duration::from_secs(4));
        assert_eq!(backoff(5, 0), Duration::from_secs(8));
    }

    #[test]
    fn backoff_caps_at_eight_seconds() {
        assert_eq!(backoff(6, 0), Duration::from_secs(8));
        assert_eq!(backoff(u32::MAX, 0), Duration::from_secs(8));
    }

    #[test]
    fn jitter_is_added_but_bounded() {
        assert_eq!(backoff(2, 250), Duration::from_millis(1_250));
        assert_eq!(backoff(2, 10_000), Duration::from_millis(1_500));
    }
}
