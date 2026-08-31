use super::*;
use crate::domain::profile::Profile;

fn fixture() -> Profile {
    let raw = include_str!("../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn mac_fixture() -> Profile {
    let raw = include_str!("../../../../profiles/macos_m2_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn has(flags: &[String], prefix: &str) -> bool {
    flags.iter().any(|f| f.starts_with(prefix))
}

#[test]
fn chrome_version_switch_present_when_set() {
    let p = mac_fixture();
    assert!(
        p.chrome_version.is_some(),
        "fixture should have chrome_version"
    );
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
    let ua = flags
        .iter()
        .find(|f| f.starts_with("--user-agent="))
        .unwrap();
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
    let ua = flags
        .iter()
        .find(|f| f.starts_with("--user-agent="))
        .unwrap();
    assert!(ua.contains("Chrome/135"), "UA unchanged when no override");
}

#[test]
fn accept_lang_switch_carries_no_quality_values() {
    // Chromium's --accept-lang parser CHECK-fails on ';' or ' ' and aborts the
    // browser at startup, so the header-form value in the profile must be
    // reduced to a bare list before it reaches the switch.
    let flags = profile_to_flags(&fixture());
    let accept = flags
        .iter()
        .find(|f| f.starts_with("--accept-lang="))
        .expect("--accept-lang emitted");

    assert!(
        !accept.contains(';') && !accept.contains(' '),
        "would abort the browser: {accept}"
    );
    assert_eq!(accept, "--accept-lang=en-US,en");
}

#[test]
fn accept_lang_value_strips_q_values_and_whitespace() {
    assert_eq!(accept_lang_switch_value("en-US,en;q=0.9"), "en-US,en");
    assert_eq!(
        accept_lang_switch_value("fr-FR, fr;q=0.9, en;q=0.8"),
        "fr-FR,fr,en"
    );
    assert_eq!(accept_lang_switch_value("de-DE"), "de-DE");
}
