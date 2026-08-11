use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::input::{
    DispatchMouseEventParams, DispatchMouseEventType,
};
use rand::Rng;

use super::Point;

const DELTA_MIN: f64 = 40.0;
const DELTA_MAX: f64 = 120.0;
const PIXELS_PER_SCROLL: f64 = 600.0;

struct ScrollPlan {
    deltas: Vec<f64>,
    delays_ms: Vec<u64>,
}

fn plan_scroll(distance: f64) -> ScrollPlan {
    let mut rng = rand::rng();
    let total = distance.abs();
    let direction = if distance >= 0.0 { 1.0 } else { -1.0 };
    let mut deltas = Vec::new();
    let mut delays_ms = Vec::new();
    let mut scrolled = 0.0;

    while scrolled < total {
        let remaining = total - scrolled;
        let base_delta = rng.random_range(DELTA_MIN..=DELTA_MAX);
        let step = base_delta.min(remaining);
        deltas.push(step * direction);
        scrolled += step;
        let tick = rng.random_range(25u64..=60);
        let extra = if rng.random_bool(0.12) {
            rng.random_range(150u64..=400)
        } else {
            0
        };
        delays_ms.push(tick + extra);
    }
    ScrollPlan { deltas, delays_ms }
}

pub async fn smooth_scroll(page: &Page, cursor: &Point, distance: f64) {
    let plan = plan_scroll(distance);
    for (delta, delay) in plan.deltas.iter().zip(plan.delays_ms.iter()) {
        let mut cmd =
            DispatchMouseEventParams::new(DispatchMouseEventType::MouseWheel, cursor.x, cursor.y);
        cmd.delta_x = Some(0.0);
        cmd.delta_y = Some(*delta);
        let _ = page.execute(cmd).await;
        tokio::time::sleep(Duration::from_millis(*delay)).await;
    }
}

pub async fn scroll_to_bottom(page: &Page, cursor: &Point, times: u32) {
    for _ in 0..times.max(1) {
        smooth_scroll(page, cursor, PIXELS_PER_SCROLL).await;
        tokio::time::sleep(Duration::from_millis(300)).await;
    }
}

pub async fn scroll_to_element(page: &Page, selector: &str, cursor: &Point) {
    let js = format!(
        r#"(() => {{
  const el = document.querySelector({});
  if (!el) return 0;
  const r = el.getBoundingClientRect();
  return r.top;
}})()"#,
        serde_json::to_string(selector).unwrap_or_default()
    );
    let val = page.evaluate(js).await.ok();
    let offset: f64 = val.and_then(|v| v.into_value().ok()).unwrap_or(0.0);
    if offset.abs() > 10.0 {
        smooth_scroll(page, cursor, offset).await;
    }
}
