use super::*;
use crate::domain::profile::Profile;

fn mac_fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/macos_m2_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn has(flags: &[String], prefix: &str) -> bool {
    flags.iter().any(|f| f.starts_with(prefix))
}

#[test]
fn battery_switches_present_when_set() {
    let p = mac_fixture();
    assert!(p.hardware.battery.is_some(), "fixture should have battery");
    let flags = profile_to_flags(&p);
    assert!(has(&flags, "--cosmium-battery-charging="), "charging");
    assert!(has(&flags, "--cosmium-battery-level="), "level");
    assert!(
        has(&flags, "--cosmium-battery-charging-time="),
        "charging_time"
    );
    assert!(
        has(&flags, "--cosmium-battery-discharging-time="),
        "discharging_time"
    );
}

#[test]
fn battery_switches_absent_when_none() {
    let mut p = mac_fixture();
    p.hardware.battery = None;
    let flags = profile_to_flags(&p);
    assert!(
        !has(&flags, "--cosmium-battery-"),
        "no battery switches when battery is None"
    );
}
