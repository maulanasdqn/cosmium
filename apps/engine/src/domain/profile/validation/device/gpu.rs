use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

use super::platform::platform_of;

pub fn webgl_renderer(p: &Profile) -> Vec<Diagnostic> {
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

pub fn gpu_vendor_matches_platform(p: &Profile) -> Vec<Diagnostic> {
    let Some(os) = platform_of(&p.identity) else {
        return vec![];
    };
    let r = &p.gpu.renderer;
    let v = &p.gpu.vendor;
    let mut out = Vec::new();

    let mentions_apple = r.contains("Apple") || v.contains("Apple") || r.contains("Metal");
    let mentions_direct3d = r.contains("Direct3D") || r.contains("D3D11");
    let mentions_mesa = r.contains("Mesa") || r.contains("llvmpipe");

    match os {
        "macOS" => check_macos(mentions_apple, mentions_direct3d, mentions_mesa, &mut out),
        "Windows" => check_windows(mentions_apple, mentions_direct3d, mentions_mesa, &mut out),
        "Linux" => check_linux(mentions_apple, mentions_direct3d, &mut out),
        _ => {}
    }
    out
}

fn check_macos(apple: bool, d3d: bool, mesa: bool, out: &mut Vec<Diagnostic>) {
    if d3d {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "macOS profile cannot report Direct3D — that's Windows ANGLE backend",
        ));
    }
    if mesa {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "macOS profile cannot report Mesa/llvmpipe — that's Linux software stack",
        ));
    }
    if !apple {
        out.push(Diagnostic::warn(
            "gpu.renderer",
            "macOS GPU renderer usually mentions Apple/Metal",
        ));
    }
}

fn check_windows(apple: bool, d3d: bool, mesa: bool, out: &mut Vec<Diagnostic>) {
    if apple {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "Windows profile cannot report Apple GPU",
        ));
    }
    if mesa {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "Windows profile cannot report Mesa/llvmpipe",
        ));
    }
    if !d3d {
        out.push(Diagnostic::warn(
            "gpu.renderer",
            "Windows ANGLE renderer usually contains Direct3D11/D3D11",
        ));
    }
}

fn check_linux(apple: bool, d3d: bool, out: &mut Vec<Diagnostic>) {
    if apple {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "Linux profile cannot report Apple GPU",
        ));
    }
    if d3d {
        out.push(Diagnostic::err(
            "gpu.renderer",
            "Linux profile cannot report Direct3D",
        ));
    }
}
