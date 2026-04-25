use super::{Diagnostic, Profile};

pub(super) fn screen_dimensions(p: &Profile) -> Vec<Diagnostic> {
    let s = &p.screen;
    let mut out = Vec::new();
    if s.avail_width > s.width {
        out.push(Diagnostic::err(
            "screen.avail_width",
            format!("avail_width ({}) > width ({})", s.avail_width, s.width),
        ));
    }
    if s.avail_height > s.height {
        out.push(Diagnostic::err(
            "screen.avail_height",
            format!("avail_height ({}) > height ({})", s.avail_height, s.height),
        ));
    }
    if s.width == 0 || s.height == 0 {
        out.push(Diagnostic::err(
            "screen",
            "width/height must be > 0 (Xvfb default 0 is a tell)",
        ));
    }
    out
}

pub(super) fn pixel_depth(p: &Profile) -> Vec<Diagnostic> {
    if p.screen.color_depth != p.screen.pixel_depth {
        vec![Diagnostic::err(
            "screen.pixel_depth",
            "real browsers report color_depth == pixel_depth",
        )]
    } else {
        vec![]
    }
}

pub(super) fn hardware_concurrency(p: &Profile) -> Vec<Diagnostic> {
    let n = p.hardware.hardware_concurrency;
    let mut out = Vec::new();
    if n < 2 {
        out.push(Diagnostic::err(
            "hardware.hardware_concurrency",
            "must be >= 2; single-core devices barely exist on real Chrome",
        ));
    }
    if n % 2 != 0 {
        out.push(Diagnostic::warn(
            "hardware.hardware_concurrency",
            format!("{n} is odd — most real CPUs report even core counts"),
        ));
    }
    out
}

pub(super) fn device_memory(p: &Profile) -> Vec<Diagnostic> {
    let m = p.hardware.device_memory_gb;
    let allowed = [0.25_f32, 0.5, 1.0, 2.0, 4.0, 8.0];
    if !allowed.iter().any(|a| (a - m).abs() < f32::EPSILON) {
        vec![Diagnostic::err(
            "hardware.device_memory_gb",
            format!("Chrome rounds deviceMemory to {allowed:?}; got {m}"),
        )]
    } else {
        vec![]
    }
}

pub(super) fn webgl_renderer(p: &Profile) -> Vec<Diagnostic> {
    let r = &p.gpu.renderer;
    if r.contains("SwiftShader") || r.contains("0x0000C0DE") {
        vec![Diagnostic::err(
            "gpu.renderer",
            "renderer must not advertise SwiftShader — that string alone busts the fingerprint",
        )]
    } else {
        vec![]
    }
}

pub(super) fn canvas_noise_seed(p: &Profile) -> Vec<Diagnostic> {
    let seed = &p.canvas_noise.seed;
    if seed.len() != 32 || !seed.chars().all(|c| c.is_ascii_hexdigit()) {
        vec![Diagnostic::err(
            "canvas_noise.seed",
            "must be exactly 32 hex characters (16-byte seed)",
        )]
    } else {
        vec![]
    }
}
