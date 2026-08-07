use super::*;
use crate::domain::profile::*;

fn fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

fn mac_fixture() -> Profile {
    let raw = include_str!("../../../../../../profiles/macos_m2_en-us.json");
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

#[test]
fn macos_fixture_is_coherent() {
    let p = mac_fixture();
    let diags = validate(&p);
    let errors: Vec<_> = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    assert!(
        errors.is_empty(),
        "macOS fixture should be error-free, got: {errors:#?}"
    );
}

#[test]
fn detects_apple_gpu_on_windows_profile() {
    let mut p = fixture();
    p.gpu.renderer = "ANGLE (Apple, ANGLE Metal Renderer: Apple M2, Unspecified Version)".into();
    p.gpu.vendor = "Google Inc. (Apple)".into();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "gpu.renderer"),
        "should reject Apple GPU on Windows UA, got: {diags:#?}"
    );
}

#[test]
fn detects_d3d_renderer_on_mac_profile() {
    let mut p = mac_fixture();
    p.gpu.renderer =
        "ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0, D3D11)".into();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "gpu.renderer"),
    );
}

#[test]
fn detects_mesa_on_mac_profile() {
    let mut p = mac_fixture();
    p.gpu.renderer = "ANGLE (Intel, Mesa Intel(R) UHD Graphics (CML GT2), OpenGL ES 3.2)".into();
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "gpu.renderer"),
        "should reject Mesa renderer on macOS UA"
    );
}

#[test]
fn detects_segoe_ui_on_macos_profile() {
    let mut p = mac_fixture();
    p.fonts.installed.push("Segoe UI".into());
    let diags = validate(&p);
    assert!(diags.iter().any(|d| d.severity == Severity::Error
        && d.field == "fonts.installed"
        && d.message.contains("Segoe UI")),);
}

#[test]
fn detects_linux_fonts_on_macos_profile() {
    let mut p = mac_fixture();
    p.fonts.installed.push("DejaVu Sans".into());
    p.fonts.installed.push("Liberation Sans".into());
    let diags = validate(&p);
    let leak_diags: Vec<_> = diags
        .iter()
        .filter(|d| d.severity == Severity::Error && d.field == "fonts.installed")
        .collect();
    assert!(
        leak_diags.len() >= 2,
        "expected >=2 leak diagnostics, got {leak_diags:#?}"
    );
}

#[test]
fn detects_missing_platform_staples() {
    let mut p = mac_fixture();
    p.fonts.installed = vec!["Custom Font".into()];
    let diags = validate(&p);
    assert!(diags.iter().any(|d| d.severity == Severity::Error
        && d.field == "fonts.installed"
        && d.message.contains("staples")),);
}

#[test]
fn detects_touch_points_zero_on_mobile_ua() {
    let mut p = fixture();
    p.identity.user_agent =
        "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Mobile Safari/537.36"
            .into();
    p.hardware.max_touch_points = 0;
    let diags = validate(&p);
    assert!(
        diags
            .iter()
            .any(|d| d.severity == Severity::Error && d.field == "hardware.max_touch_points"),
    );
}

#[test]
fn detects_implausible_hardware_combo() {
    let mut p = fixture();
    p.hardware.hardware_concurrency = 16;
    p.hardware.device_memory_gb = 0.5;
    let diags = validate(&p);
    assert!(diags.iter().any(|d| d.severity == Severity::Warning && d.field == "hardware"));
}
