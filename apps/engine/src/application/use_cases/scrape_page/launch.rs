use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use anyhow::{Context, Result};

use crate::application::use_cases::stealth_config::build_stealth_config;
use crate::domain::profile::Profile;
use crate::domain::runtime::browser::LaunchSpec;
use crate::domain::runtime::{
    fontconfig_for, force_proxied_webrtc, profile_to_env, profile_to_flags, user_data_dir_for,
};
use crate::domain::scraping::port::BrowserSession;
use crate::domain::scraping::request::ProxyConfig;
use crate::infrastructure::runtime::CdpSessionRuntime;
use crate::infrastructure::scraping::pool::PooledBrowser;
use crate::infrastructure::scraping::{ChromiumScraper, ProxyForwarder};

use super::ScrapePageInput;

const STRIP_AUTOMATION_TELLS: &str = "--cosmium-strip-automation-tells";

pub(super) fn pool_key(profile_name: &str, proxy: Option<&str>) -> String {
    user_data_dir_for(profile_name, proxy)
        .to_string_lossy()
        .into_owned()
}

pub(super) fn config_id(input: &ScrapePageInput, profile: &Profile) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.binary.hash(&mut hasher);
    input.headful.hash(&mut hasher);
    input.geo_sync.hash(&mut hasher);
    serde_json::to_string(profile)
        .unwrap_or_default()
        .hash(&mut hasher);
    hasher.finish()
}

pub(super) async fn launch(
    input: &ScrapePageInput,
    mut profile: Profile,
    proxy: Option<ProxyConfig>,
    config_id: u64,
) -> Result<PooledBrowser> {
    let proxy_url = proxy.as_ref().map(|p| p.url.as_str());
    if input.geo_sync {
        if let Some(tz) = crate::infrastructure::geo::exit_timezone(proxy_url).await {
            profile.locale.timezone = tz;
        }
    }

    let mut flags = base_flags(input, &profile, proxy.is_some());
    let forwarder = match &proxy {
        Some(p) => ProxyForwarder::start(p).await,
        None => None,
    };
    match (&forwarder, &proxy) {
        (Some(fwd), _) => flags.push(fwd.chrome_flag()),
        (None, Some(p)) => flags.push(format!("--proxy-server={}", p.url)),
        (None, None) => {}
    }

    let session: Arc<dyn BrowserSession> = Arc::new(CdpSessionRuntime::new());
    let spec = LaunchSpec {
        flags,
        urls: Vec::new(),
        env: profile_to_env(&profile),
        user_data_dir: Some(user_data_dir_for(&profile.name, proxy_url)),
        fontconfig: Some(fontconfig_for(&profile)),
    };
    let endpoint = session
        .launch_with_cdp(&input.binary, spec)
        .await
        .with_context(|| format!("launching {}", input.binary.display()))?;
    tracing::info!(profile = %profile.name, proxied = proxy.is_some(), "launched pooled browser");

    let scraper =
        ChromiumScraper::from_browser(endpoint.browser, Some(build_stealth_config(&profile)));
    Ok(PooledBrowser::new(scraper, session, forwarder, config_id))
}

fn base_flags(input: &ScrapePageInput, profile: &Profile, proxied: bool) -> Vec<String> {
    let mut flags = profile_to_flags(profile);
    if !input.headful {
        flags.push("--headless=new".into());
    }
    if !flags
        .iter()
        .any(|f| f.contains("cosmium-strip-automation-tells"))
    {
        flags.push(STRIP_AUTOMATION_TELLS.into());
    }
    if proxied {
        force_proxied_webrtc(&mut flags);
    }
    flags
}
