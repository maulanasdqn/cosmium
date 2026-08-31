use std::time::{Duration, Instant};

use chromiumoxide::Page;
use rand::Rng;

use super::behavior::mouse;
use super::behavior::scroll;

const NAV_TIMEOUT: Duration = Duration::from_secs(20);
const COOKIE_POLL: Duration = Duration::from_secs(2);
const COOKIE_BUDGET: Duration = Duration::from_secs(30);
const POST_WARMUP_PAUSE_MS: u64 = 3000;

pub async fn warmup_homepage(page: &Page, target_url: &str) -> bool {
    let Some(home) = extract_home_url(target_url) else {
        return false;
    };
    tracing::info!(url = %home, "warming up homepage");

    let initial_cookie = get_datadome_cookie(page).await;
    navigate(page, &home).await;
    tracing::debug!("homepage loaded, settling 3s");
    tokio::time::sleep(Duration::from_secs(3)).await;

    tracing::debug!("simulating mouse/scroll presence");
    match tokio::time::timeout(Duration::from_secs(15), simulate_presence(page)).await {
        Ok(()) => tracing::debug!("mouse/scroll simulation done"),
        Err(_) => tracing::debug!("mouse/scroll simulation timed out (15s), continuing"),
    }

    tracing::debug!("waiting for DataDome cookie to rotate (c.js)");
    wait_for_cookie_rotation(page, initial_cookie.as_deref()).await;

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

async fn wait_for_cookie_rotation(page: &Page, initial: Option<&str>) {
    let start = Instant::now();
    let initial_val = initial.unwrap_or_default().to_owned();

    let page_url = page.url().await.ok().flatten();
    let explicit_urls = page_url.as_ref().map(|u| vec![u.clone()]);

    while start.elapsed() < COOKIE_BUDGET {
        tokio::time::sleep(COOKIE_POLL).await;
        let cookie = get_datadome_cookie_for_urls(page, explicit_urls.clone()).await;
        if let Some(ref current) = cookie {
            if *current != initial_val {
                tracing::info!("DataDome cookie rotated — c.js resolved");
                tokio::time::sleep(Duration::from_secs(2)).await;
                return;
            }
        }
        let elapsed = start.elapsed().as_secs();
        if elapsed % 10 == 0 && elapsed > 0 {
            tracing::debug!(elapsed, "waiting for DataDome cookie rotation");
        }
    }
    tracing::warn!("DataDome cookie did not rotate within budget (30s)");
}

async fn get_datadome_cookie(page: &Page) -> Option<String> {
    get_datadome_cookie_for_urls(page, None).await
}

async fn get_datadome_cookie_for_urls(page: &Page, urls: Option<Vec<String>>) -> Option<String> {
    use chromiumoxide::cdp::browser_protocol::network::GetCookiesParams;

    let params = GetCookiesParams { urls };
    let timeout = Duration::from_secs(5);
    let result = match tokio::time::timeout(timeout, page.execute(params)).await {
        Ok(Ok(resp)) => resp,
        _ => return None,
    };
    result
        .result
        .cookies
        .iter()
        .find(|c| c.name == "datadome")
        .map(|c| c.value.clone())
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
