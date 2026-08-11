use std::time::{Duration, Instant};

use chromiumoxide::Page;

use crate::domain::scraping::detection::is_challenge_page;

const POLL_INTERVAL: Duration = Duration::from_millis(750);

pub async fn wait_past_challenge(page: &Page, timeout: Duration) -> String {
    let deadline = Instant::now() + timeout;
    let mut clicked_waf = false;
    loop {
        let html = match page.content().await {
            Ok(h) => h,
            Err(_) => return String::new(),
        };
        if !is_challenge_page(&html) {
            return html;
        }
        if !clicked_waf && is_aws_waf(&html) {
            tracing::info!("AWS WAF challenge detected, clicking Begin");
            click_waf_begin(page).await;
            clicked_waf = true;
            tokio::time::sleep(Duration::from_secs(5)).await;
            continue;
        }
        if Instant::now() >= deadline {
            tracing::warn!("challenge did not resolve before timeout");
            return html;
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

fn is_aws_waf(html: &str) -> bool {
    html.contains("awswaf.com") || html.contains("gokuProps")
}

async fn click_waf_begin(page: &Page) {
    let js = r#"
        (function() {
            var btn = document.querySelector('#captcha-container button');
            if (!btn) btn = document.querySelector('button');
            if (btn) { btn.click(); return 'clicked'; }
            return 'no-button';
        })()
    "#;
    match page.evaluate(js).await {
        Ok(v) => {
            let result = v.into_value::<String>().unwrap_or_default();
            tracing::info!(result, "WAF begin button");
        }
        Err(e) => tracing::warn!(error = %e, "failed to click WAF button"),
    }
}
