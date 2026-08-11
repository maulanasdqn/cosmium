mod challenge;
mod cookies;

pub use challenge::{wait_for_challenge_js, wait_for_resolution};
pub use cookies::{clear_cookies, get_dd_cookie_value, preseed_cookies, save_cookies};

use std::time::Duration;

pub(crate) const POLL_INTERVAL: Duration = Duration::from_millis(1000);
pub(crate) const CHALLENGE_BUDGET: Duration = Duration::from_secs(30);

const HARD_BLOCK_MARKERS: &[&str] = &[
    "access is temporarily restricted",
    "automated (bot) activity",
];

const SOFT_CHALLENGE_MARKERS: &[&str] = &[
    "captcha-delivery.com",
    "datadome.co",
    "confirm you are human",
    "begin",
];

#[derive(Debug)]
pub enum DdVerdict {
    Clean,
    SoftChallenge,
    HardBlock,
}

pub fn classify(html: &str) -> DdVerdict {
    let lower = html.to_lowercase();
    if HARD_BLOCK_MARKERS.iter().any(|m| lower.contains(m)) {
        return DdVerdict::HardBlock;
    }
    if SOFT_CHALLENGE_MARKERS.iter().any(|m| lower.contains(m)) {
        return DdVerdict::SoftChallenge;
    }
    DdVerdict::Clean
}

pub fn url_host(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))?;
    rest.split('/').next().map(|h| h.to_owned())
}
