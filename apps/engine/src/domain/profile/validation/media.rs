use super::{Diagnostic, Profile};
use crate::domain::profile::MediaDeviceKind;

pub(super) fn media_devices(p: &Profile) -> Vec<Diagnostic> {
    if p.media_devices.is_empty() {
        return vec![Diagnostic::err(
            "media_devices",
            "must contain at least one device — empty enumerateDevices() is a container tell",
        )];
    }
    let mut out = Vec::new();
    let has_audio_input = p
        .media_devices
        .iter()
        .any(|d| matches!(d.kind, MediaDeviceKind::AudioInput));
    let has_audio_output = p
        .media_devices
        .iter()
        .any(|d| matches!(d.kind, MediaDeviceKind::AudioOutput));
    if !has_audio_input {
        out.push(Diagnostic::warn(
            "media_devices",
            "no audioinput device — real desktops almost always have one",
        ));
    }
    if !has_audio_output {
        out.push(Diagnostic::warn(
            "media_devices",
            "no audiooutput device — real desktops almost always have one",
        ));
    }
    out
}
