use super::*;
use crate::domain::profile::Profile;

fn fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn mac_fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/macos_m2_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn has(flags: &[String], prefix: &str) -> bool {
    flags.iter().any(|f| f.starts_with(prefix))
}

#[test]
fn emits_all_cosmium_switches() {
    let flags = profile_to_flags(&fixture());
    let want = [
        "--cosmium-platform=Win32",
        "--cosmium-ua-platform=Windows",
        "--cosmium-languages=en-US,en",
        "--cosmium-hardware-concurrency=12",
        "--cosmium-device-memory=8",
        "--cosmium-color-depth=24",
        "--cosmium-max-touch-points=0",
        "--cosmium-webgl-vendor=Google Inc. (NVIDIA)",
    ];
    for w in want {
        assert!(flags.iter().any(|f| f == w), "missing flag: {w}");
    }
}

#[test]
fn webgl_renderer_passed_through() {
    let flags = profile_to_flags(&fixture());
    assert!(has(&flags, "--cosmium-webgl-renderer=ANGLE (NVIDIA"));
}

#[test]
fn hygiene_flags_present() {
    let flags = profile_to_flags(&fixture());
    // Note: --disable-blink-features=AutomationControlled was removed after
    // patch 0013 disabled AutomationControlled at source level, and patch 0014
    // suppresses the bad-flags infobar that the flag used to trigger.
    for w in [
        "--no-default-browser-check",
        "--no-first-run",
        "--no-pings",
        "--disable-domain-reliability",
        "--disable-component-update",
        "--disable-search-engine-choice-screen",
        "--password-store=basic",
        "--use-mock-keychain",
    ] {
        assert!(flags.iter().any(|f| f == w), "missing flag: {w}");
    }
}

#[test]
fn disable_features_includes_telemetry_surfaces() {
    let flags = profile_to_flags(&fixture());
    let df = flags.iter().find(|f| f.starts_with("--disable-features=")).unwrap();
    for feat in ["Translate", "PrivacySandboxAdsAPIs", "AcceptCHFrame"] {
        assert!(df.contains(feat), "--disable-features missing {feat}: {df}");
    }
}

#[test]
fn audio_latency_switches_present() {
    let flags = profile_to_flags(&fixture());
    assert!(has(&flags, "--cosmium-audio-base-latency="));
    assert!(has(&flags, "--cosmium-audio-output-latency="));
}

#[test]
fn canvas_seed_switch_present() {
    let flags = profile_to_flags(&fixture());
    assert!(has(&flags, "--cosmium-canvas-seed="), "canvas seed required");
}

#[test]
fn strip_automation_tells_is_opt_in() {
    let mut p = fixture();
    p.strip_automation_tells = false;
    assert!(!profile_to_flags(&p).iter().any(|f| f == "--cosmium-strip-automation-tells"));
    p.strip_automation_tells = true;
    assert!(profile_to_flags(&p).iter().any(|f| f == "--cosmium-strip-automation-tells"));
}

#[test]
fn webrtc_policy_passed_through() {
    let flags = profile_to_flags(&mac_fixture());
    assert!(flags.iter().any(|f| f == "--force-webrtc-ip-handling-policy=default_public_interface_only"));
}

#[test]
fn chrome_version_switch_present_when_set() {
    let p = mac_fixture();
    assert!(p.chrome_version.is_some(), "fixture should have chrome_version");
    let flags = profile_to_flags(&p);
    assert!(
        has(&flags, "--cosmium-chrome-version="),
        "chrome version switch required"
    );
}

#[test]
fn chrome_version_rewrites_user_agent() {
    let mut p = mac_fixture();
    p.chrome_version = Some("151.0.7922.108".into());
    let flags = profile_to_flags(&p);
    let ua = flags.iter().find(|f| f.starts_with("--user-agent=")).unwrap();
    assert!(
        ua.contains("Chrome/151.0.0.0"),
        "UA should contain spoofed Chrome/151.0.0.0, got: {ua}"
    );
    assert!(
        !ua.contains("Chrome/135"),
        "UA should not contain old Chrome/135, got: {ua}"
    );
}

#[test]
fn chrome_version_absent_when_none() {
    let mut p = mac_fixture();
    p.chrome_version = None;
    let flags = profile_to_flags(&p);
    assert!(
        !has(&flags, "--cosmium-chrome-version="),
        "no switch when chrome_version is None"
    );
    // UA should be unchanged
    let ua = flags.iter().find(|f| f.starts_with("--user-agent=")).unwrap();
    assert!(ua.contains("Chrome/135"), "UA unchanged when no override");
}

#[test]
fn audio_sample_rate_and_channels_present() {
    let flags = profile_to_flags(&fixture());
    assert!(has(&flags, "--cosmium-audio-sample-rate="), "audio sample rate");
    assert!(has(&flags, "--cosmium-audio-max-channels="), "audio max channels");
}

#[test]
fn screen_avail_switches_present() {
    let flags = profile_to_flags(&mac_fixture());
    assert!(has(&flags, "--cosmium-avail-left="), "avail_left");
    assert!(has(&flags, "--cosmium-avail-top="), "avail_top");
}
