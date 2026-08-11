use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::input::{
    DispatchMouseEventParams, DispatchMouseEventType, MouseButton,
};
use rand::Rng;

use super::{BoundingBox, Point, element_box};

const MOVE_STEPS_MIN: usize = 18;
const MOVE_STEPS_MAX: usize = 38;
const STEP_DELAY_MIN_US: u64 = 4000;
const STEP_DELAY_MAX_US: u64 = 12000;
const CLICK_DOWN_MIN_MS: u64 = 40;
const CLICK_DOWN_MAX_MS: u64 = 120;

fn cubic_bezier(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;
    Point {
        x: uu * u * p0.x + 3.0 * uu * t * p1.x + 3.0 * u * tt * p2.x + tt * t * p3.x,
        y: uu * u * p0.y + 3.0 * uu * t * p1.y + 3.0 * u * tt * p2.y + tt * t * p3.y,
    }
}

fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

struct MovePlan {
    path: Vec<Point>,
    delays_us: Vec<u64>,
    hold_ms: u64,
}

fn plan_move(from: Point, to: Point) -> MovePlan {
    let mut rng = rand::rng();
    let steps = rng.random_range(MOVE_STEPS_MIN..=MOVE_STEPS_MAX);
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let cp1 = Point {
        x: from.x + dx * rng.random_range(0.1..0.4) + rng.random_range(-30.0..30.0),
        y: from.y + dy * rng.random_range(0.0..0.6) + rng.random_range(-30.0..30.0),
    };
    let cp2 = Point {
        x: from.x + dx * rng.random_range(0.6..0.9) + rng.random_range(-20.0..20.0),
        y: from.y + dy * rng.random_range(0.4..1.0) + rng.random_range(-20.0..20.0),
    };
    let mut path = Vec::with_capacity(steps + 1);
    let mut delays_us = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = ease_out_quad(i as f64 / steps as f64);
        let mut pt = cubic_bezier(from, cp1, cp2, to, t);
        if i > 0 && i < steps {
            pt.x += rng.random_range(-0.5..0.5);
            pt.y += rng.random_range(-0.5..0.5);
        }
        path.push(pt);
        delays_us.push(rng.random_range(STEP_DELAY_MIN_US..=STEP_DELAY_MAX_US));
    }
    let hold_ms = rng.random_range(CLICK_DOWN_MIN_MS..=CLICK_DOWN_MAX_MS);
    MovePlan {
        path,
        delays_us,
        hold_ms,
    }
}

async fn execute_move(page: &Page, plan: &MovePlan) {
    for (pt, &delay) in plan.path.iter().zip(plan.delays_us.iter()) {
        let cmd = DispatchMouseEventParams::new(DispatchMouseEventType::MouseMoved, pt.x, pt.y);
        let _ = page.execute(cmd).await;
        tokio::time::sleep(Duration::from_micros(delay)).await;
    }
}

async fn press_click(page: &Page, pt: Point, hold_ms: u64) {
    let mut down = DispatchMouseEventParams::new(DispatchMouseEventType::MousePressed, pt.x, pt.y);
    down.button = Some(MouseButton::Left);
    down.click_count = Some(1);
    let _ = page.execute(down).await;

    tokio::time::sleep(Duration::from_millis(hold_ms)).await;

    let mut up = DispatchMouseEventParams::new(DispatchMouseEventType::MouseReleased, pt.x, pt.y);
    up.button = Some(MouseButton::Left);
    up.click_count = Some(1);
    let _ = page.execute(up).await;
}

fn pick_target(bbox: &BoundingBox) -> Point {
    let mut rng = rand::rng();
    bbox.random_point(&mut rng)
}

pub async fn click(page: &Page, selector: &str, cursor: &mut Point) {
    let Some(bbox) = element_box(page, selector).await else {
        return;
    };
    let target = pick_target(&bbox);
    let plan = plan_move(*cursor, target);
    execute_move(page, &plan).await;
    press_click(page, target, plan.hold_ms).await;
    *cursor = target;
}

pub async fn click_box(page: &Page, bbox: &BoundingBox, cursor: &mut Point) {
    let target = pick_target(bbox);
    let plan = plan_move(*cursor, target);
    execute_move(page, &plan).await;
    press_click(page, target, plan.hold_ms).await;
    *cursor = target;
}

pub fn initial_cursor(vw: f64, vh: f64) -> Point {
    let mut rng = rand::rng();
    Point {
        x: rng.random_range(vw * 0.3..vw * 0.7),
        y: rng.random_range(vh * 0.3..vh * 0.7),
    }
}
