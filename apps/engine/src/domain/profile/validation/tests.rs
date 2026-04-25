use super::*;
use crate::domain::profile::*;

fn fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

#[test]
fn win11_fixture_is_coherent() {
    let p = fixture();
    let diags = validate(&p);
    let errors: Vec<_> = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "fixture should be error-free, got: {errors:#?}"
    );
}

#[test]
fn detects_ua_platform_mismatch() {
    let mut p = fixture();
    p.identity.client_hints.platform = "Linux".to_owned();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "identity.client_hints.platform")
    );
}

#[test]
fn detects_swiftshader_renderer() {
    let mut p = fixture();
    p.gpu.renderer = "ANGLE (Google, SwiftShader Device, ...)".to_owned();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "gpu.renderer")
    );
}

#[test]
fn detects_accept_language_mismatch() {
    let mut p = fixture();
    p.locale.accept_language = "fr-FR,fr;q=0.9".to_owned();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "locale.accept_language")
    );
}

#[test]
fn detects_invalid_timezone() {
    let mut p = fixture();
    p.locale.timezone = "Mars/Olympus_Mons".to_owned();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "locale.timezone")
    );
}
