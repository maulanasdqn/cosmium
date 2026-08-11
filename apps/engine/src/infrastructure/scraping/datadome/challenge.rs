use std::time::{Duration, Instant};

use chromiumoxide::Page;

use super::cookies::get_dd_cookie_value;
use super::{CHALLENGE_BUDGET, DdVerdict, POLL_INTERVAL, classify};

const RELOAD_SETTLE: Duration = Duration::from_secs(5);

pub async fn wait_for_challenge_js(page: &Page, initial_cookie: Option<&str>) -> DdVerdict {
    let start = Instant::now();
    let initial = initial_cookie.unwrap_or_default().to_owned();

    tracing::info!("waiting for DataDome c.js fingerprint check");
    while start.elapsed() < CHALLENGE_BUDGET {
        tokio::time::sleep(POLL_INTERVAL).await;

        let current_cookie = get_dd_cookie_value(page).await;
        if let Some(ref cookie_val) = current_cookie {
            if !initial.is_empty() && cookie_val != &initial {
                tracing::info!("DataDome cookie rotated — c.js completed");
                tokio::time::sleep(Duration::from_secs(2)).await;

                if let Some(html) = cdp_content_with_timeout(page, Duration::from_secs(5)).await {
                    let verdict = classify(&html);
                    if matches!(verdict, DdVerdict::Clean) {
                        tracing::info!("DataDome challenge resolved after c.js");
                        return DdVerdict::Clean;
                    }
                    tracing::debug!("DataDome cookie changed but still on challenge page");
                }
            }
        }

        if let Ok(Some(url)) = page.url().await {
            if !url.contains("captcha-delivery") && !url.contains("geo.captcha") {
                if let Some(html) = cdp_content_with_timeout(page, Duration::from_secs(5)).await {
                    if !html.contains("captcha-delivery.com") {
                        tracing::info!("navigated away from DataDome challenge page");
                        return DdVerdict::Clean;
                    }
                }
            }
        }

        let elapsed = start.elapsed().as_secs();
        if elapsed % 5 == 0 && elapsed > 0 {
            tracing::debug!(elapsed, "still waiting for DataDome c.js");
        }
    }

    tracing::warn!("DataDome c.js wait budget exhausted (30s)");
    DdVerdict::SoftChallenge
}

pub async fn wait_for_resolution(page: &Page) -> (String, DdVerdict) {
    let start = Instant::now();
    let mut reloaded = false;

    loop {
        let html = match cdp_content_with_timeout(page, Duration::from_secs(10)).await {
            Some(h) => h,
            None => {
                tracing::debug!("page.content() slow, switching to cookie-based DataDome wait");
                let initial = get_dd_cookie_value(page).await;
                let verdict = wait_for_challenge_js(page, initial.as_deref()).await;
                let html = cdp_content_with_timeout(page, Duration::from_secs(10))
                    .await
                    .unwrap_or_default();
                return (html, verdict);
            }
        };

        let verdict = classify(&html);
        match verdict {
            DdVerdict::Clean => {
                tracing::info!("DataDome page resolved — no challenge");
                return (html, DdVerdict::Clean);
            }
            DdVerdict::HardBlock => {
                tracing::warn!("DataDome hard block detected — IP flagged");
                return (html, DdVerdict::HardBlock);
            }
            DdVerdict::SoftChallenge => {
                if start.elapsed() >= CHALLENGE_BUDGET {
                    tracing::warn!("DataDome challenge budget exhausted (30s)");
                    return (html, DdVerdict::SoftChallenge);
                }
                let elapsed = start.elapsed().as_secs();
                tracing::debug!(elapsed, "DataDome soft challenge present");

                if !reloaded && start.elapsed() >= Duration::from_secs(15) {
                    tracing::info!("attempting DataDome page reload with cookies");
                    let _ = page.reload().await;
                    tokio::time::sleep(RELOAD_SETTLE).await;
                    reloaded = true;
                    continue;
                }
            }
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

async fn cdp_content_with_timeout(page: &Page, timeout: Duration) -> Option<String> {
    match tokio::time::timeout(timeout, page.content()).await {
        Ok(Ok(h)) => Some(h),
        _ => None,
    }
}
