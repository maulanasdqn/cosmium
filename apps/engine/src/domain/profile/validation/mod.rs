mod device;
mod fonts;
mod identity;
mod locale;
mod media;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_voice;

use super::Profile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub field: &'static str,
    pub message: String,
}

impl Diagnostic {
    pub(super) fn err(field: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            field,
            message: msg.into(),
        }
    }

    pub(super) fn warn(field: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            field,
            message: msg.into(),
        }
    }
}

pub fn validate(profile: &Profile) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    out.extend(identity::platform(profile));
    out.extend(identity::chrome_version(profile));
    out.extend(identity::brands(profile));
    out.extend(locale::accept_language(profile));
    out.extend(locale::languages(profile));
    out.extend(locale::timezone(profile));
    out.extend(locale::voices(profile));
    out.extend(locale::voice_defaults(profile));
    out.extend(device::screen_dimensions(profile));
    out.extend(device::pixel_depth(profile));
    out.extend(device::hardware_concurrency(profile));
    out.extend(device::device_memory(profile));
    out.extend(device::webgl_renderer(profile));
    out.extend(device::canvas_noise_seed(profile));
    out.extend(device::gpu_vendor_matches_platform(profile));
    out.extend(device::touch_points_match_form_factor(profile));
    out.extend(device::ram_cores_plausible(profile));
    out.extend(fonts::platform_staples_present(profile));
    out.extend(fonts::no_cross_platform_leak(profile));
    out.extend(media::media_devices(profile));
    out
}
