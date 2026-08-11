use super::*;
use crate::domain::profile::*;

fn fixture() -> Profile {
    let raw = include_str!("../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

#[test]
fn detects_no_default_voice() {
    let mut p = fixture();
    for v in &mut p.voices {
        v.default = false;
    }
    let diags = validate(&p);
    assert!(
        diags.iter().any(|d| d.severity == Severity::Error
            && d.field == "voices"
            && d.message.contains("no voice has default=true")),
        "should detect missing default voice, got: {diags:#?}"
    );
}

#[test]
fn detects_multiple_default_voices() {
    let mut p = fixture();
    for v in &mut p.voices {
        v.default = true;
    }
    let diags = validate(&p);
    assert!(
        diags.iter().any(|d| d.severity == Severity::Error
            && d.field == "voices"
            && d.message.contains("default=true")),
        "should detect multiple default voices, got: {diags:#?}"
    );
}
