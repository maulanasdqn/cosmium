use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::fetch::{
    ContinueRequestParams, EnableParams, EventRequestPaused, FailRequestParams, RequestPattern,
    RequestStage,
};
use chromiumoxide::cdp::browser_protocol::network::ErrorReason;
use futures_util::StreamExt;

const BLOCKED_DOMAINS: &[&str] = &[
    "google-analytics.com",
    "googletagmanager.com",
    "googlesyndication.com",
    "doubleclick.net",
    "facebook.net",
    "facebook.com/tr",
    "connect.facebook.net",
    "hotjar.com",
    "clarity.ms",
    "segment.io",
    "segment.com",
    "sentry.io",
    "newrelic.com",
    "nr-data.net",
    "amplitude.com",
    "mixpanel.com",
    "optimizely.com",
];

const BLOCKED_EXTENSIONS: &[&str] = &[".woff2", ".woff", ".ttf", ".otf"];

pub struct NetworkInterceptor {
    task: tokio::task::JoinHandle<()>,
}

impl NetworkInterceptor {
    pub async fn start(page: &Page) -> Option<Self> {
        let patterns = vec![
            RequestPattern::builder()
                .request_stage(RequestStage::Request)
                .build(),
        ];

        if let Err(e) = page
            .execute(EnableParams::builder().patterns(patterns).build())
            .await
        {
            tracing::warn!(error = %e, "failed to enable Fetch interception");
            return None;
        }

        let mut listener = match page.event_listener::<EventRequestPaused>().await {
            Ok(l) => l,
            Err(e) => {
                tracing::warn!(error = %e, "failed to attach Fetch listener");
                return None;
            }
        };

        let page_clone = page.clone();

        let task = tokio::spawn(async move {
            while let Some(event) = listener.next().await {
                if event.response_status_code.is_some() {
                    let _ = page_clone
                        .execute(ContinueRequestParams::new(event.request_id.clone()))
                        .await;
                    continue;
                }

                let url = &event.request.url;
                if should_block(url) {
                    let _ = page_clone
                        .execute(FailRequestParams::new(
                            event.request_id.clone(),
                            ErrorReason::BlockedByClient,
                        ))
                        .await;
                    tracing::debug!(url = %url, "blocked tracking request");
                    continue;
                }

                let _ = page_clone
                    .execute(ContinueRequestParams::new(event.request_id.clone()))
                    .await;
            }
        });

        tracing::info!("network interceptor started, blocking trackers & fonts");
        Some(Self { task })
    }
}

impl Drop for NetworkInterceptor {
    fn drop(&mut self) {
        self.task.abort();
    }
}

fn should_block(url: &str) -> bool {
    let lower = url.to_lowercase();
    BLOCKED_DOMAINS.iter().any(|d| lower.contains(d))
        || BLOCKED_EXTENSIONS.iter().any(|e| lower.ends_with(e))
}

#[cfg(test)]
mod tests {
    use super::should_block;

    #[test]
    fn blocks_tracking() {
        assert!(should_block(
            "https://www.google-analytics.com/analytics.js"
        ));
        assert!(should_block(
            "https://connect.facebook.net/en_US/fbevents.js"
        ));
        assert!(should_block("https://fonts.gstatic.com/s/inter.woff2"));
    }

    #[test]
    fn allows_normal() {
        assert!(!should_block("https://www.tokopedia.com/api/v1/product"));
        assert!(!should_block("https://cdn.example.com/image.jpg"));
    }
}
