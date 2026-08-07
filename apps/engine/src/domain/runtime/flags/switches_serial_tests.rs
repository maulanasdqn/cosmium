use super::*;
use crate::domain::profile::Profile;

fn fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn flag_value<'a>(flags: &'a [String], prefix: &str) -> &'a str {
    let f = flags.iter().find(|f| f.starts_with(prefix)).unwrap();
    &f[prefix.len()..]
}

#[test]
fn voices_switch_is_valid_json() {
    let flags = profile_to_flags(&fixture());
    let json = flag_value(&flags, "--cosmium-voices=");
    let parsed: Vec<serde_json::Value> =
        serde_json::from_str(json).expect("voices must be valid JSON");
    assert!(!parsed.is_empty());
}

#[test]
fn fonts_switch_includes_arial() {
    let flags = profile_to_flags(&fixture());
    let csv = flag_value(&flags, "--cosmium-fonts=");
    assert!(csv.contains("Arial"), "fonts list must include Arial");
}

#[test]
fn media_devices_switch_is_valid_json() {
    let flags = profile_to_flags(&fixture());
    let json = flag_value(&flags, "--cosmium-media-devices=");
    let parsed: Vec<serde_json::Value> =
        serde_json::from_str(json).expect("media_devices must be valid JSON");
    assert!(!parsed.is_empty());
}
