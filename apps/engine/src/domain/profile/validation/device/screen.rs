use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

pub fn screen_dimensions(p: &Profile) -> Vec<Diagnostic> {
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

pub fn pixel_depth(p: &Profile) -> Vec<Diagnostic> {
    if p.screen.color_depth != p.screen.pixel_depth {
        vec![Diagnostic::err(
            "screen.pixel_depth",
            "real browsers report color_depth == pixel_depth",
        )]
    } else {
        vec![]
    }
}
