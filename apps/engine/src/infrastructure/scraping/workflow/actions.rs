use std::time::Duration;

use chromiumoxide::Page;

use crate::infrastructure::scraping::behavior;
use crate::infrastructure::scraping::behavior::Point;

const SCROLL_SETTLE_MS: u64 = 1200;

pub async fn click(page: &Page, selector: &str, cursor: &mut Point) {
    behavior::mouse::click(page, selector, cursor).await;
}

pub async fn input(page: &Page, selector: &str, text: &str, cursor: &mut Point) {
    behavior::mouse::click(page, selector, cursor).await;
    tokio::time::sleep(Duration::from_millis(80)).await;
    behavior::keyboard::clear_field(page, selector).await;
    behavior::keyboard::type_text(page, text).await;
}

pub async fn scroll(
    page: &Page,
    infinite: bool,
    selector: Option<&str>,
    times: u32,
    cursor: &mut Point,
) {
    for _ in 0..times.max(1) {
        if infinite {
            behavior::scroll::scroll_to_bottom(page, cursor, 1).await;
        } else if let Some(target) = selector {
            behavior::scroll::scroll_to_element(page, target, cursor).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
            behavior::mouse::click(page, target, cursor).await;
        }
        tokio::time::sleep(Duration::from_millis(SCROLL_SETTLE_MS)).await;
    }
}
