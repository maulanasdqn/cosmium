use crate::application::use_cases::stealth_config::build_stealth_config;
use crate::domain::profile::Profile;

fn fixture() -> Profile {
    let raw = include_str!("../../../profiles/win11_rtx3060_en-us.json");
    serde_json::from_str(raw).expect("fixture parses")
}

#[test]
fn webgl_script_carries_profile_gpu_and_leaves_no_placeholder() {
    let p = fixture();
    let js = build_stealth_config(&p).webgl_script();
    assert!(!js.contains("__VENDOR__") && !js.contains("__RENDERER__"));
    assert!(js.contains(&p.gpu.vendor), "vendor spoof present");
    assert!(js.contains(&p.gpu.renderer), "renderer spoof present");
    assert!(js.contains("37445") && js.contains("37446"));
    assert!(js.contains("WebGLRenderingContext") && js.contains("WebGL2RenderingContext"));
}

#[test]
fn webgl_vendor_is_json_escaped() {
    let mut cfg = build_stealth_config(&fixture());
    cfg.webgl_renderer = r#"ANGLE ("quoted", \back)"#.to_owned();
    let js = cfg.webgl_script();
    assert!(
        js.contains(r#"\"quoted\""#),
        "renderer quotes escaped: {js}"
    );
    assert!(!js.contains("__RENDERER__"));
}

#[test]
fn canvas_script_embeds_seed() {
    let cfg = build_stealth_config(&fixture());
    let js = cfg.canvas_script();
    assert!(!js.contains("__SEED__"));
    assert!(js.contains(&cfg.noise_seed.to_string()), "seed embedded");
    assert!(js.contains("getImageData") && js.contains("toDataURL") && js.contains("toBlob"));
}

#[test]
fn audio_script_embeds_seed_and_base_latency() {
    let p = fixture();
    let cfg = build_stealth_config(&p);
    let js = cfg.audio_script();
    assert!(!js.contains("__SEED__") && !js.contains("__BL__"));
    assert!(
        js.contains(&format!("{}", p.audio.base_latency)),
        "base latency embedded"
    );
    assert!(js.contains("getChannelData"), "channel-data noise present");
}
