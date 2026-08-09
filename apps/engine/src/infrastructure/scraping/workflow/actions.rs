use std::time::Duration;

use chromiumoxide::Page;

use super::js_string;

const SCROLL_SETTLE_MS: u64 = 1200;

pub async fn click(page: &Page, selector: &str) {
    let code = format!(
        "(() => {{ \
            const el = document.querySelector({sel}); \
            if (el) {{ el.click(); return true; }} \
            return false; \
        }})()",
        sel = js_string(selector)
    );
    page.evaluate(code).await.ok();
}

pub async fn input(page: &Page, selector: &str, text: &str) {
    let code = format!(
        "(() => {{ \
            const el = document.querySelector({sel}); \
            if (!el) return false; \
            el.focus(); \
            el.value = {val}; \
            el.dispatchEvent(new Event('input', {{ bubbles: true }})); \
            el.dispatchEvent(new Event('change', {{ bubbles: true }})); \
            return true; \
        }})()",
        sel = js_string(selector),
        val = js_string(text)
    );
    page.evaluate(code).await.ok();
}

pub async fn scroll(page: &Page, infinite: bool, selector: Option<&str>, times: u32) {
    for _ in 0..times.max(1) {
        if infinite {
            page.evaluate(
                "(() => { \
                    window.scrollTo(0, document.body.scrollHeight); \
                    return true; \
                })()",
            )
            .await
            .ok();
        } else if let Some(target) = selector {
            click(page, target).await;
        }
        tokio::time::sleep(Duration::from_millis(SCROLL_SETTLE_MS)).await;
    }
}
