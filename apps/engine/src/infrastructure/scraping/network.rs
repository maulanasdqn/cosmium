use std::sync::Arc;
use std::time::{Duration, Instant};

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::network::{
    EnableParams, EventResponseReceived, GetResponseBodyParams, RequestId,
};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

pub struct ApiCapture {
    pattern: String,
    captured: Arc<Mutex<Vec<CapturedRequest>>>,
    _task: JoinHandle<()>,
}

struct CapturedRequest {
    url: String,
    request_id: RequestId,
}

impl ApiCapture {
    pub async fn start(page: &Page, url_pattern: &str) -> Option<Self> {
        page.execute(EnableParams::default()).await.ok()?;
        let mut listener = page.event_listener::<EventResponseReceived>().await.ok()?;

        let pattern = url_pattern.to_owned();
        let captured = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&captured);
        let pat = pattern.clone();

        let _task = tokio::spawn(async move {
            while let Some(event) = listener.next().await {
                if event.response.url.contains(&pat) {
                    tracing::debug!(url = %event.response.url, "captured API response");
                    sink.lock().await.push(CapturedRequest {
                        url: event.response.url.clone(),
                        request_id: event.request_id.clone(),
                    });
                }
            }
        });

        tracing::info!(pattern = %url_pattern, "API capture started");
        Some(Self {
            pattern,
            captured,
            _task,
        })
    }

    pub async fn wait_and_collect(
        self,
        page: &Page,
        timeout: Duration,
    ) -> Vec<(String, String)> {
        let start = Instant::now();
        let poll = Duration::from_millis(500);

        while start.elapsed() < timeout {
            {
                let items = self.captured.lock().await;
                if !items.is_empty() {
                    break;
                }
            }
            tokio::time::sleep(poll).await;
        }

        self._task.abort();
        tokio::time::sleep(Duration::from_millis(200)).await;

        let items = self.captured.lock().await;
        if items.is_empty() {
            tracing::warn!(
                pattern = %self.pattern,
                "no API responses captured within timeout"
            );
            return Vec::new();
        }

        let mut results = Vec::new();
        for item in items.iter() {
            let params = GetResponseBodyParams::new(item.request_id.clone());
            match page.execute(params).await {
                Ok(resp) => {
                    let body = if resp.result.base64_encoded {
                        use base64::Engine;
                        let engine = base64::engine::general_purpose::STANDARD;
                        match engine.decode(&resp.result.body) {
                            Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                            Err(_) => resp.result.body,
                        }
                    } else {
                        resp.result.body
                    };
                    tracing::info!(
                        url = %item.url,
                        bytes = body.len(),
                        "collected API response body"
                    );
                    results.push((item.url.clone(), body));
                }
                Err(e) => {
                    tracing::warn!(
                        url = %item.url,
                        error = %e,
                        "failed to get API response body"
                    );
                }
            }
        }

        results
    }
}
