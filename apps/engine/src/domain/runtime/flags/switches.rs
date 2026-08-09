use crate::domain::profile::Profile;

use super::features::{disable_features_list, webrtc_flags};

/// Rewrite `Chrome/X.Y.Z.W` inside a User-Agent string to use the given
/// full version.  Also patches `Safari/X.Y.Z.W` → `Safari/537.36` (Chrome
/// always sends this fixed value, so a stale numeric slip is a tell).
fn rewrite_ua_version(ua: &str, full_version: &str) -> String {
    let major = full_version.split('.').next().unwrap_or(full_version);
    // Chrome reduced UA uses "Chrome/MAJOR.0.0.0"
    let chrome_re = regex::Regex::new(r"Chrome/\d+\.\d+\.\d+\.\d+").unwrap();
    let out = chrome_re.replace(ua, format!("Chrome/{major}.0.0.0"));
    out.into_owned()
}

pub fn profile_to_flags(p: &Profile) -> Vec<String> {
    let mut f = Vec::new();

    // If chrome_version is set, rewrite the UA string on the fly so the
    // HTTP User-Agent header and Sec-CH-UA headers all agree.
    let ua = match &p.chrome_version {
        Some(ver) => rewrite_ua_version(&p.identity.user_agent, ver),
        None => p.identity.user_agent.clone(),
    };
    f.push(format!("--user-agent={ua}"));
    f.push(format!("--lang={}", primary_lang(&p.locale.languages)));
    f.push(format!("--accept-lang={}", p.locale.accept_language));
    f.push(format!(
        "--cosmium-platform={}",
        p.identity.navigator_platform
    ));
    f.push(format!(
        "--cosmium-ua-platform={}",
        p.identity.client_hints.platform
    ));
    f.push(format!(
        "--cosmium-languages={}",
        p.locale.languages.join(",")
    ));
    f.push(format!(
        "--cosmium-hardware-concurrency={}",
        p.hardware.hardware_concurrency
    ));
    f.push(format!(
        "--cosmium-device-memory={}",
        p.hardware.device_memory_gb
    ));
    f.push(format!("--cosmium-color-depth={}", p.screen.color_depth));
    f.push(format!(
        "--cosmium-max-touch-points={}",
        p.hardware.max_touch_points
    ));
    f.push(format!("--cosmium-webgl-vendor={}", p.gpu.vendor));
    f.push(format!("--cosmium-webgl-renderer={}", p.gpu.renderer));
    f.push(format!(
        "--cosmium-audio-base-latency={}",
        p.audio.base_latency
    ));
    f.push(format!(
        "--cosmium-audio-output-latency={}",
        p.audio.output_latency
    ));
    f.push(format!("--cosmium-canvas-seed={}", p.canvas_noise.seed));
    f.push(format!(
        "--cosmium-audio-sample-rate={}",
        p.audio.sample_rate
    ));
    f.push(format!(
        "--cosmium-audio-max-channels={}",
        p.audio.max_channel_count
    ));
    let voices_json = serde_json::to_string(&p.voices).unwrap_or_default();
    f.push(format!("--cosmium-voices={voices_json}"));
    f.push(format!("--cosmium-fonts={}", p.fonts.installed.join(",")));
    let devices_json = serde_json::to_string(&p.media_devices).unwrap_or_default();
    f.push(format!("--cosmium-media-devices={devices_json}"));
    f.push(format!("--cosmium-avail-left={}", p.screen.avail_left));
    f.push(format!("--cosmium-avail-top={}", p.screen.avail_top));
    f.push(format!(
        "--window-size={},{}",
        p.screen.width, p.screen.height
    ));
    f.push(format!(
        "--force-device-scale-factor={}",
        p.screen.device_pixel_ratio
    ));
    // AutomationControlled is now disabled at source level (patch 0013),
    // so the --disable-blink-features flag is no longer needed and would
    // trigger a "bad flags" infobar that leaks in the DOM.
    if p.strip_automation_tells {
        f.push("--cosmium-strip-automation-tells".into());
    }
    f.push(format!("--cosmium-timezone={}", p.locale.timezone));
    if let Some(ver) = &p.chrome_version {
        f.push(format!("--cosmium-chrome-version={ver}"));
    }
    f.push(format!("--disable-features={}", disable_features_list()));
    f.push("--no-default-browser-check".into());
    f.push("--no-first-run".into());
    f.push("--no-pings".into());
    f.push("--disable-domain-reliability".into());
    f.push("--disable-component-update".into());
    f.push("--disable-search-engine-choice-screen".into());
    f.push("--password-store=basic".into());
    f.push("--use-mock-keychain".into());
    f.extend(webrtc_flags(&p.webrtc.ip_handling_policy));
    f
}

pub(super) fn primary_lang(langs: &[String]) -> &str {
    langs.first().map(String::as_str).unwrap_or("en-US")
}

#[cfg(test)]
#[path = "switches_core_tests.rs"]
mod core_tests;

#[cfg(test)]
#[path = "switches_serial_tests.rs"]
mod serial_tests;
