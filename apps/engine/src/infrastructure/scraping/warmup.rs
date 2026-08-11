use std::time::{Duration, Instant};

use chromiumoxide::Page;
use rand::Rng;

use super::behavior::mouse;
use super::behavior::scroll;

const NAV_TIMEOUT: Duration = Duration::from_secs(20);
const SETTLE: Duration = Duration::from_secs(3);
const CHALLENGE_BUDGET: Duration = Duration::from_secs(20);
const POST_WARMUP_PAUSE_MS: u64 = 3000;

pub async fn warmup_homepage(page: &Page, target_url: &str) -> bool {
    let Some(home) = extract_home_url(target_url) else {
        return false;
    };
    tracing::info!(url = %home, "warming up homepage");
    navigate(page, &home).await;
    tracing::debug!("homepage loaded, settling 3s");
    tokio::time::sleep(Duration::from_secs(3)).await;
    tracing::debug!("simulating mouse/scroll presence");
    match tokio::time::timeout(Duration::from_secs(15), simulate_presence(page)).await {
        Ok(()) => tracing::debug!("mouse/scroll simulation done"),
        Err(_) => tracing::debug!("mouse/scroll simulation timed out (15s), continuing"),
    }
    tracing::debug!("waiting for warmup challenge to clear");
    wait_until_clean(page).await;
    let extra = { rand::rng().random_range(0..1500) as u64 };
    tracing::debug!(pause_ms = POST_WARMUP_PAUSE_MS + extra, "post-warmup pause");
    tokio::time::sleep(Duration::from_millis(POST_WARMUP_PAUSE_MS + extra)).await;
    tracing::info!("warmup complete");
    true
}

pub async fn click_navigate(page: &Page, target_url: &str) -> bool {
    let safe = target_url.replace('\'', "\\'");
    let js = format!(
        r"(() => {{ const a = document.createElement('a'); a.href = '{safe}'; \
        a.style.position = 'fixed'; a.style.top = '10px'; a.style.left = '10px'; \
        a.style.zIndex = '99999'; a.textContent = 'go'; \
        document.body.appendChild(a); a.click(); }})()"
    );
    let ok = tokio::time::timeout(Duration::from_secs(5), page.evaluate(js))
        .await
        .is_ok_and(|r| r.is_ok());
    if ok {
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    ok
}

async fn simulate_presence(page: &Page) {
    let mut cursor = mouse::initial_cursor(1920.0, 1080.0);
    let (targets, scroll_dist, final_pause) = {
        let mut rng = rand::rng();
        let t: [(f64, f64); 3] = [
            (
                rng.random_range(300.0..700.0),
                rng.random_range(200.0..500.0),
            ),
            (
                rng.random_range(600.0..1000.0),
                rng.random_range(300.0..600.0),
            ),
            (
                rng.random_range(400.0..800.0),
                rng.random_range(350.0..550.0),
            ),
        ];
        (
            t,
            rng.random_range(80.0..200.0),
            rng.random_range(200..500) as u64,
        )
    };

    for (tx, ty) in targets {
        let bbox = super::behavior::BoundingBox {
            x: tx - 10.0,
            y: ty - 10.0,
            width: 20.0,
            height: 20.0,
        };
        mouse::click_box(page, &bbox, &mut cursor).await;
        let pause = { rand::rng().random_range(150..350) as u64 };
        tokio::time::sleep(Duration::from_millis(pause)).await;
    }

    scroll::smooth_scroll(page, &cursor, scroll_dist).await;
    tokio::time::sleep(Duration::from_millis(final_pause)).await;
}

async fn wait_until_clean(page: &Page) {
    let start = Instant::now();
    loop {
        tokio::time::sleep(SETTLE).await;
        let html = match tokio::time::timeout(Duration::from_secs(10), page.content()).await {
            Ok(Ok(h)) => h,
            Ok(Err(_)) => {
                tracing::debug!("warmup page.content() error");
                return;
            }
            Err(_) => {
                tracing::debug!("warmup page.content() timed out");
                return;
            }
        };
        if !crate::domain::scraping::detection::is_challenge_page(&html) {
            tracing::debug!("warmup page clean — no challenge detected");
            return;
        }
        let elapsed = start.elapsed().as_secs();
        tracing::debug!(elapsed, "warmup challenge still present");
        if start.elapsed() >= CHALLENGE_BUDGET {
            tracing::debug!("warmup challenge budget exhausted, proceeding");
            return;
        }
    }
}

pub async fn cdp_content_timeout(page: &Page, timeout: Duration) -> Option<String> {
    match tokio::time::timeout(timeout, page.content()).await {
        Ok(Ok(h)) => Some(h),
        _ => None,
    }
}

async fn navigate(page: &Page, url: &str) {
    match tokio::time::timeout(NAV_TIMEOUT, page.goto(url)).await {
        Ok(Ok(_)) => {}
        Ok(Err(_)) => {}
        Err(_) => {}
    }
}

fn extract_home_url(target_url: &str) -> Option<String> {
    let rest = target_url
        .strip_prefix("https://")
        .or_else(|| target_url.strip_prefix("http://"))?;
    let scheme = if target_url.starts_with("https") {
        "https"
    } else {
        "http"
    };
    let host = rest.split('/').next().filter(|h| !h.is_empty())?;
    Some(format!("{scheme}://{host}/"))
}

#[cfg(test)]
mod tests {
    use super::extract_home_url;

    #[test]
    fn extracts_home_url_from_deep_path() {
        assert_eq!(
            extract_home_url("https://www.traveloka.com/en-id/hotel/detail?spec=abc"),
            Some("https://www.traveloka.com/".to_owned())
        );
    }

    #[test]
    fn returns_none_for_garbage() {
        assert_eq!(extract_home_url("not-a-url"), None);
    }
}
