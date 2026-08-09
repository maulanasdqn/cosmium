use std::sync::Arc;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::network::{
    EnableParams, EventResponseReceived, ResourceType,
};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

const DEFAULT_STATUS: i32 = 200;

pub struct StatusWatcher {
    documents: Arc<Mutex<Vec<(String, i32)>>>,
    task: JoinHandle<()>,
}

impl StatusWatcher {
    pub async fn attach(page: &Page) -> Option<Self> {
        page.execute(EnableParams::default()).await.ok()?;
        let mut listener = page.event_listener::<EventResponseReceived>().await.ok()?;

        let documents = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&documents);
        let task = tokio::spawn(async move {
            while let Some(event) = listener.next().await {
                record_document(&sink, &event).await;
            }
        });

        Some(Self { documents, task })
    }

    pub async fn status_for(&self, final_url: &str) -> i32 {
        self.task.abort();
        let documents = self.documents.lock().await;
        documents
            .iter()
            .rev()
            .find(|(url, _)| url == final_url)
            .or_else(|| documents.last())
            .map_or(DEFAULT_STATUS, |(_, status)| *status)
    }
}

async fn record_document(sink: &Arc<Mutex<Vec<(String, i32)>>>, event: &EventResponseReceived) {
    if event.r#type != ResourceType::Document {
        return;
    }
    let status = i32::try_from(event.response.status).unwrap_or(DEFAULT_STATUS);
    sink.lock().await.push((event.response.url.clone(), status));
}
