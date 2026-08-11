use crate::domain::profile::Profile;

use super::features::{disable_features_list, webrtc_flags};

/// Strip quality values from an Accept-Language list.
///
/// `locale.accept_language` is stored in HTTP header form (`en-US,en;q=0.9`)
/// because that is what it means on the wire, but Chromium's `--accept-lang`
/// switch parses a bare comma-separated list. Feeding it a q-value trips a
/// CHECK in net/http/http_util.cc:
///
///   Check failed: std::string::npos == language.find_first_of("; ")
///
/// which aborts the browser during startup rather than failing softly, so
/// every `cosmium run` with a header-form profile died before loading a page.
fn accept_lang_switch_value(accept_language: &str) -> String {
    accept_language
        .split(',')
        .filter_map(|part| {
            let lang = part.split(';').next().unwrap_or("").trim();
            (!lang.is_empty()).then(|| lang.to_string())
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn rewrite_ua_version(ua: &str, full_version: &str) -> String {
    let major = full_version.split('.').next().unwrap_or(full_version);
    let chrome_re = regex::Regex::new(r"Chrome/\d+\.\d+\.\d+\.\d+").unwrap();
    let out = chrome_re.replace(ua, format!("Chrome/{major}.0.0.0"));
    out.into_owned()
}

pub fn profile_to_flags(p: &Profile) -> Vec<String> {
    let mut f = Vec::new();

    let ua = match &p.chrome_version {
        Some(ver) => rewrite_ua_version(&p.identity.user_agent, ver),
        None => p.identity.user_agent.clone(),
    };
    f.push(format!("--user-agent={ua}"));
    f.push(format!("--lang={}", primary_lang(&p.locale.languages)));
    f.push(format!(
        "--accept-lang={}",
        accept_lang_switch_value(&p.locale.accept_language)
    ));
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
    // The screen dimensions need their own switches. --window-size below sizes
    // the window, but screen.width/height report the display and stay at the
    // headless 800x600 without these, contradicting both the profile and the
    // avail_* pair emitted just above.
    f.push(format!("--cosmium-screen-width={}", p.screen.width));
    f.push(format!("--cosmium-screen-height={}", p.screen.height));
    f.push(format!("--cosmium-avail-width={}", p.screen.avail_width));
    f.push(format!("--cosmium-avail-height={}", p.screen.avail_height));
    f.push(format!(
        "--window-size={},{}",
        p.screen.width, p.screen.height
    ));
    f.push(format!(
        "--force-device-scale-factor={}",
        p.screen.device_pixel_ratio
    ));
    if p.strip_automation_tells {
        f.push("--cosmium-strip-automation-tells".into());
    }
    f.push(format!("--cosmium-timezone={}", p.locale.timezone));
    if let Some(ver) = &p.chrome_version {
        f.push(format!("--cosmium-chrome-version={ver}"));
    }
    if let Some(bat) = &p.hardware.battery {
        f.push(format!("--cosmium-battery-charging={}", bat.charging));
        f.push(format!("--cosmium-battery-level={}", bat.level));
        match bat.charging_time_seconds {
            Some(t) => f.push(format!("--cosmium-battery-charging-time={t}")),
            None => f.push("--cosmium-battery-charging-time=Infinity".into()),
        }
        match bat.discharging_time_seconds {
            Some(t) => f.push(format!("--cosmium-battery-discharging-time={t}")),
            None => f.push("--cosmium-battery-discharging-time=Infinity".into()),
        }
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
    f.push("--disable-blink-features=AutomationControlled".into());
    f.push("--disable-infobars".into());
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
#[path = "switches_battery_tests.rs"]
mod battery_tests;

#[cfg(test)]
#[path = "switches_serial_tests.rs"]
mod serial_tests;
