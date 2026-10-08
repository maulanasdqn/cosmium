use crate::domain::profile::Profile;

use super::features::{disable_features_list, webrtc_flags};

pub fn accept_lang_switch_value(accept_language: &str) -> String {
    accept_language
        .split(',')
        .filter_map(|part| {
            let lang = part.split(';').next().unwrap_or("").trim();
            (!lang.is_empty()).then(|| lang.to_owned())
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn rewrite_ua_version(ua: &str, full_version: &str) -> String {
    let major = full_version.split('.').next().unwrap_or(full_version);
    let Ok(chrome_re) = regex::Regex::new(r"Chrome/\d+\.\d+\.\d+\.\d+") else {
        return ua.to_owned();
    };
    let out = chrome_re.replace(ua, format!("Chrome/{major}.0.0.0"));
    out.into_owned()
}

pub fn user_agent_for(p: &Profile) -> String {
    p.chrome_version.as_ref().map_or_else(
        || p.identity.user_agent.clone(),
        |ver| rewrite_ua_version(&p.identity.user_agent, ver),
    )
}

fn push_identity(f: &mut Vec<String>, p: &Profile) {
    f.push(format!("--user-agent={}", user_agent_for(p)));
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
}

fn push_hardware(f: &mut Vec<String>, p: &Profile) {
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
    let pointer = if p.identity.client_hints.mobile {
        "coarse"
    } else {
        "fine"
    };
    f.push(format!("--cosmium-pointer={pointer}"));
    f.push(format!("--cosmium-webgl-vendor={}", p.gpu.vendor));
    f.push(format!("--cosmium-webgl-renderer={}", p.gpu.renderer));
    if let Some(limits) = p.gpu.webgl_limits_switch() {
        f.push(format!("--cosmium-webgl-limits={limits}"));
    }
    if let Some(excluded) = p.gpu.webgl_excluded_extensions_switch() {
        f.push(format!("--cosmium-webgl-exclude-extensions={excluded}"));
    }
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
}

fn push_battery(f: &mut Vec<String>, p: &Profile) {
    let Some(bat) = &p.hardware.battery else {
        return;
    };
    f.push(format!("--cosmium-battery-charging={}", bat.charging));
    f.push(format!("--cosmium-battery-level={}", bat.level));
    f.push(bat.charging_time_seconds.map_or_else(
        || "--cosmium-battery-charging-time=Infinity".into(),
        |t| format!("--cosmium-battery-charging-time={t}"),
    ));
    f.push(bat.discharging_time_seconds.map_or_else(
        || "--cosmium-battery-discharging-time=Infinity".into(),
        |t| format!("--cosmium-battery-discharging-time={t}"),
    ));
}

fn screen_info_switch(s: &crate::domain::profile::Screen) -> String {
    let right = s
        .width
        .saturating_sub(s.avail_width.saturating_add(s.avail_left));
    let bottom = s
        .height
        .saturating_sub(s.avail_height.saturating_add(s.avail_top));
    format!(
        "--screen-info={{0,0 {}x{} colorDepth={} devicePixelRatio={} workAreaLeft={} workAreaTop={} workAreaRight={right} workAreaBottom={bottom}}}",
        s.width, s.height, s.color_depth, s.device_pixel_ratio, s.avail_left, s.avail_top
    )
}

pub fn profile_to_flags(p: &Profile) -> Vec<String> {
    let mut f = Vec::new();

    push_identity(&mut f, p);
    push_hardware(&mut f, p);
    let voices_json = serde_json::to_string(&p.voices).unwrap_or_default();
    f.push(format!("--cosmium-voices={voices_json}"));
    f.push(format!("--cosmium-fonts={}", p.fonts.installed.join(",")));
    let devices_json = serde_json::to_string(&p.media_devices).unwrap_or_default();
    f.push(format!("--cosmium-media-devices={devices_json}"));
    f.push(format!("--cosmium-avail-left={}", p.screen.avail_left));
    f.push(format!("--cosmium-avail-top={}", p.screen.avail_top));
    f.push(format!("--cosmium-screen-width={}", p.screen.width));
    f.push(format!("--cosmium-screen-height={}", p.screen.height));
    f.push(format!("--cosmium-avail-width={}", p.screen.avail_width));
    f.push(format!("--cosmium-avail-height={}", p.screen.avail_height));
    f.push(format!(
        "--window-size={},{}",
        p.screen.avail_width, p.screen.avail_height
    ));
    f.push(format!(
        "--force-device-scale-factor={}",
        p.screen.device_pixel_ratio
    ));
    f.push(screen_info_switch(&p.screen));
    if p.strip_automation_tells {
        f.push("--cosmium-strip-automation-tells".into());
    }
    f.push(format!("--cosmium-timezone={}", p.locale.timezone));
    if let Some(ver) = &p.chrome_version {
        f.push(format!("--cosmium-chrome-version={ver}"));
    }
    push_battery(&mut f, p);
    f.push(format!("--disable-features={}", disable_features_list()));
    f.push("--disable-field-trial-config".into());
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
    f.extend(webrtc_flags(p.webrtc.ip_handling_policy));
    f
}

pub(super) fn primary_lang(langs: &[String]) -> &str {
    langs.first().map_or("en-US", String::as_str)
}

#[cfg(test)]
#[path = "switches_core_tests.rs"]
mod core_tests;

#[cfg(test)]
#[path = "switches_version_tests.rs"]
mod version_tests;

#[cfg(test)]
#[path = "switches_screen_tests.rs"]
mod screen_tests;

#[cfg(test)]
#[path = "switches_battery_tests.rs"]
mod battery_tests;

#[cfg(test)]
#[path = "switches_serial_tests.rs"]
mod serial_tests;
