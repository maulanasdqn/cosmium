use std::borrow::Cow;
use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::emulation::SetFocusEmulationEnabledParams;
use chromiumoxide::cdp::browser_protocol::network::EnableParams;
use chromiumoxide::types::{Command, Method, MethodId};
use serde::Serialize;

use super::super::stealth::StealthConfig;

const LOCAL_HOSTS: [&str; 4] = ["localhost", "127.0.0.1", "0.0.0.0", "[::1]"];
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Serialize)]
struct BlockPattern {
    #[serde(rename = "urlPattern")]
    url_pattern: String,
    block: bool,
}

#[derive(Debug, Clone, Serialize)]
struct SetBlockedUrls {
    urls: Vec<String>,
    #[serde(rename = "urlPatterns")]
    url_patterns: Vec<BlockPattern>,
}

impl Method for SetBlockedUrls {
    fn identifier(&self) -> MethodId {
        Cow::Borrowed("Network.setBlockedURLs")
    }
}

impl Command for SetBlockedUrls {
    type Response = serde_json::Value;
}

pub(super) async fn block_local_network(page: &Page) {
    let _ = page.execute(EnableParams::default()).await;
    let cmd = SetBlockedUrls {
        urls: LOCAL_HOSTS.iter().map(|h| format!("*://{h}*")).collect(),
        url_patterns: LOCAL_HOSTS
            .iter()
            .map(|h| BlockPattern {
                url_pattern: format!("*://{h}:*/*"),
                block: true,
            })
            .collect(),
    };
    if let Err(e) = page.execute(cmd).await {
        tracing::debug!(error = %e, "local network blocking unavailable");
    }
}

pub(super) async fn emulate_focus(page: &Page) {
    let _ = page
        .execute(SetFocusEmulationEnabledParams::new(true))
        .await;
}

pub(super) async fn is_native_build(page: &Page, cfg: &StealthConfig) -> bool {
    let probe = "JSON.stringify([navigator.platform, navigator.hardwareConcurrency])";
    let Ok(Ok(result)) = tokio::time::timeout(PROBE_TIMEOUT, page.evaluate(probe)).await else {
        return false;
    };
    let Ok(raw) = result.into_value::<String>() else {
        return false;
    };
    let expected =
        serde_json::json!([cfg.navigator_platform, cfg.hardware_concurrency]).to_string();
    let native = raw == expected;
    tracing::debug!(native, observed = %raw, "detected cosmium native patches");
    native
}
