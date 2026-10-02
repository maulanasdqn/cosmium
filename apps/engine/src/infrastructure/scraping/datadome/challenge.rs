use std::time::{Duration, Instant};

use chromiumoxide::Page;

use super::cookies::get_dd_cookie_value;
use super::{CHALLENGE_BUDGET, DdVerdict, POLL_INTERVAL, classify};

const RELOAD_SETTLE: Duration = Duration::from_secs(5);

pub async fn wait_for_challenge_js(page: &Page, initial_cookie: Option<&str>) -> DdVerdict {
    let start = Instant::now();
    let initial = initial_cookie.unwrap_or_default().to_owned();

    tracing::info!("waiting for the site's verification script");
    while start.elapsed() < CHALLENGE_BUDGET {
        tokio::time::sleep(POLL_INTERVAL).await;

        let current_cookie = get_dd_cookie_value(page).await;
        if let Some(ref cookie_val) = current_cookie {
            if !initial.is_empty() && cookie_val != &initial {
                tracing::info!("verification cookie updated");
                tokio::time::sleep(Duration::from_secs(2)).await;

                if let Some(html) = cdp_content_with_timeout(page, Duration::from_secs(5)).await {
                    let verdict = classify(&html);
                    if matches!(verdict, DdVerdict::Clean) {
                        tracing::info!("verification challenge resolved");
                        return DdVerdict::Clean;
                    }
                    tracing::debug!(
                        "verification cookie changed but the challenge page is still shown"
                    );
                }
            }
        }

        if let Ok(Some(url)) = page.url().await {
            if !url.contains("captcha-delivery") && !url.contains("geo.captcha") {
                if let Some(html) = cdp_content_with_timeout(page, Duration::from_secs(5)).await {
                    if !html.contains("captcha-delivery.com") {
                        tracing::info!("left the verification page");
                        return DdVerdict::Clean;
                    }
                }
            }
        }

        let elapsed = start.elapsed().as_secs();
        if elapsed % 5 == 0 && elapsed > 0 {
            tracing::debug!(elapsed, "still waiting for verification");
        }
    }

    tracing::warn!("verification did not finish within 30s");
    DdVerdict::SoftChallenge
}

pub async fn wait_for_resolution(page: &Page) -> (String, DdVerdict) {
    let start = Instant::now();
    let mut reloaded = false;

    loop {
        let Some(html) = cdp_content_with_timeout(page, Duration::from_secs(10)).await else {
            tracing::debug!("page.content() slow, switching to cookie-based wait");
            let initial = get_dd_cookie_value(page).await;
            let verdict = wait_for_challenge_js(page, initial.as_deref()).await;
            let html = cdp_content_with_timeout(page, Duration::from_secs(10))
                .await
                .unwrap_or_default();
            return (html, verdict);
        };

        let verdict = classify(&html);
        match verdict {
            DdVerdict::Clean => {
                tracing::info!("page loaded without a challenge");
                return (html, DdVerdict::Clean);
            }
            DdVerdict::HardBlock => {
                tracing::warn!("site blocked this session");
                return (html, DdVerdict::HardBlock);
            }
            DdVerdict::SoftChallenge => {
                if start.elapsed() >= CHALLENGE_BUDGET {
                    tracing::warn!("challenge did not clear within 30s");
                    return (html, DdVerdict::SoftChallenge);
                }
                let elapsed = start.elapsed().as_secs();
                tracing::debug!(elapsed, "challenge page present");

                if !reloaded && start.elapsed() >= Duration::from_secs(15) {
                    tracing::info!("reloading the page with session cookies");
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
