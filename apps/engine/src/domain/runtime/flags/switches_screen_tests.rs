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
fn screen_avail_switches_present() {
    let flags = profile_to_flags(&mac_fixture());
    assert!(has(&flags, "--cosmium-avail-left="), "avail_left");
    assert!(has(&flags, "--cosmium-avail-top="), "avail_top");
}

#[test]
fn screen_dimension_switches_match_profile() {
    // These must agree with the names screen.cc reads; a rename on either side
    // silently stops spoofing rather than failing loudly.
    let flags = profile_to_flags(&fixture());
    for want in [
        "--cosmium-screen-width=1920",
        "--cosmium-screen-height=1080",
        "--cosmium-avail-width=1920",
        "--cosmium-avail-height=1032",
    ] {
        assert!(flags.iter().any(|f| f == want), "missing flag: {want}");
    }
}
