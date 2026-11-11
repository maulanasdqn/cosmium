use std::borrow::Cow;
use std::time::Duration;

use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::emulation::SetFocusEmulationEnabledParams;
use chromiumoxide::cdp::browser_protocol::network::EnableParams;
use chromiumoxide::types::{Command, Method, MethodId};
use serde::Serialize;

use super::super::stealth::StealthConfig;

const LOCAL_HOSTS: [&str; 4] = ["localhost", "127.0.0.1", "0.0.0.0", "[::1]"];
const LOCAL_HOST_PATTERNS: [&str; 4] = ["localhost", "127.0.0.1", "0.0.0.0", r"\[\:\:1\]"];

const PRIVATE_SLASH_16S: [&str; 3] = ["10.*.*.*", "192.168.*.*", "169.254.*.*"];
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

    let private = private_hosts();
    let mut urls: Vec<String> = LOCAL_HOSTS.iter().map(|h| format!("*://{h}*")).collect();
    for pattern in &private {
        urls.push(format!("*://{pattern}*"));
    }

    let mut url_patterns: Vec<BlockPattern> = LOCAL_HOST_PATTERNS
        .iter()
        .flat_map(|h| {
            [format!("*://{h}:*/*"), format!("*://{h}/*")].map(|url_pattern| BlockPattern {
                url_pattern,
                block: true,
            })
        })
        .collect();
    for pattern in &private {
        url_patterns.push(BlockPattern {
            url_pattern: format!("*://{pattern}:*/*"),
            block: true,
        });
        url_patterns.push(BlockPattern {
            url_pattern: format!("*://{pattern}/*"),
            block: true,
        });
    }

    let cmd = SetBlockedUrls { urls, url_patterns };
    if let Err(e) = page.execute(cmd).await {
        tracing::warn!(error = %e, "local network blocking unavailable");
    }
}

fn private_hosts() -> Vec<String> {
    let rfc1918_172 = (16..=31).map(|octet| format!("172.{octet}.*.*"));
    let carrier_nat = (64..=127).map(|octet| format!("100.{octet}.*.*"));
    PRIVATE_SLASH_16S
        .iter()
        .map(|&host| host.to_owned())
        .chain(rfc1918_172)
        .chain(carrier_nat)
        .collect()
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

#[cfg(test)]
mod tests {
    use super::private_hosts;

    #[test]
    fn covers_all_of_172_16_slash_12() {
        let hosts = private_hosts();
        assert!(hosts.contains(&"172.16.*.*".to_owned()));
        assert!(hosts.contains(&"172.31.*.*".to_owned()));
        assert!(!hosts.contains(&"172.15.*.*".to_owned()));
        assert!(!hosts.contains(&"172.32.*.*".to_owned()));
    }

    #[test]
    fn covers_all_of_100_64_slash_10() {
        let hosts = private_hosts();
        assert!(hosts.contains(&"100.64.*.*".to_owned()));
        assert!(hosts.contains(&"100.127.*.*".to_owned()));
        assert!(!hosts.contains(&"100.63.*.*".to_owned()));
        assert!(!hosts.contains(&"100.128.*.*".to_owned()));
    }

    #[test]
    fn keeps_the_other_private_ranges() {
        let hosts = private_hosts();
        for host in ["10.*.*.*", "192.168.*.*", "169.254.*.*"] {
            assert!(hosts.contains(&host.to_owned()), "{host}");
        }
    }
}
