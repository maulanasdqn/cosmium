use std::sync::Arc;
use std::time::{Duration, Instant};

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::network::{
    EventLoadingFailed, EventLoadingFinished, EventRequestWillBeSent,
};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

const POLL_INTERVAL: Duration = Duration::from_millis(400);
const QUIET_FOR: Duration = Duration::from_millis(700);
const STABLE_SAMPLES: u8 = 2;

pub struct SettleWindow {
    pub floor: Duration,
    pub timeout: Duration,
}

struct Traffic {
    in_flight: i64,
    seen: u32,
    changed_at: Instant,
}

struct NetworkIdle {
    traffic: Arc<Mutex<Traffic>>,
    tasks: Vec<JoinHandle<()>>,
}

impl NetworkIdle {
    async fn watch(page: &Page) -> Option<Self> {
        let traffic = Arc::new(Mutex::new(Traffic {
            in_flight: 0,
            seen: 0,
            changed_at: Instant::now(),
        }));

        let mut started = page.event_listener::<EventRequestWillBeSent>().await.ok()?;
        let mut finished = page.event_listener::<EventLoadingFinished>().await.ok()?;
        let mut failed = page.event_listener::<EventLoadingFailed>().await.ok()?;

        let opened = Arc::clone(&traffic);
        let closed = Arc::clone(&traffic);
        let broken = Arc::clone(&traffic);

        let tasks = vec![
            tokio::spawn(async move {
                while started.next().await.is_some() {
                    count(&opened, 1).await;
                }
            }),
            tokio::spawn(async move {
                while finished.next().await.is_some() {
                    count(&closed, -1).await;
                }
            }),
            tokio::spawn(async move {
                while failed.next().await.is_some() {
                    count(&broken, -1).await;
                }
            }),
        ];

        Some(Self { traffic, tasks })
    }

    async fn is_idle(&self, quiet_for: Duration) -> bool {
        let traffic = self.traffic.lock().await;
        traffic.in_flight <= 0 && traffic.changed_at.elapsed() >= quiet_for
    }
}

impl Drop for NetworkIdle {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}

async fn count(traffic: &Arc<Mutex<Traffic>>, delta: i64) {
    let mut traffic = traffic.lock().await;
    traffic.in_flight += delta;
    if delta > 0 {
        traffic.seen += 1;
    }
    traffic.changed_at = Instant::now();
}

pub async fn wait_for_stable_content(page: &Page, initial: String, window: SettleWindow) -> String {
    let started = Instant::now();
    let deadline = started + window.timeout;
    let floor = started + window.floor;
    let traffic = NetworkIdle::watch(page).await;
    let mut html = initial;
    let mut stable: u8 = 0;

    while Instant::now() < deadline {
        tokio::time::sleep(POLL_INTERVAL).await;
        let Ok(current) = page.content().await else {
            stable = 0;
            continue;
        };
        stable = if is_stable(html.len(), current.len()) {
            stable.saturating_add(1)
        } else {
            0
        };
        html = current;
        if stable < STABLE_SAMPLES || Instant::now() < floor {
            continue;
        }
        match &traffic {
            Some(t) if !t.is_idle(QUIET_FOR).await => stable = 0,
            _ => break,
        }
    }

    tracing::debug!(
        settled_bytes = html.len(),
        waited_ms = started.elapsed().as_millis(),
        "page settled"
    );
    html
}

fn is_stable(previous: usize, current: usize) -> bool {
    if current == 0 {
        return false;
    }
    current.abs_diff(previous) * 100 <= previous
}
