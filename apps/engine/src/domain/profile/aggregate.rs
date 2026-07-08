use serde::{Deserialize, Serialize};

use super::audio::Audio;
use super::canvas_noise::CanvasNoise;
use super::fonts::Fonts;
use super::gpu::Gpu;
use super::hardware::Hardware;
use super::identity::Identity;
use super::locale::Locale;
use super::media_devices::MediaDevice;
use super::screen::Screen;
use super::voices::Voice;
use super::webrtc::WebRtc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,

    #[serde(default = "default_version")]
    pub version: u32,

    pub identity: Identity,
    pub locale: Locale,
    pub hardware: Hardware,
    pub gpu: Gpu,
    pub screen: Screen,
    pub audio: Audio,
    pub media_devices: Vec<MediaDevice>,
    pub voices: Vec<Voice>,
    pub fonts: Fonts,
    pub webrtc: WebRtc,
    pub canvas_noise: CanvasNoise,

    /// Opt-in: emit `--cosmium-strip-automation-tells`, which the CDP-leak
    /// strip patch (0013) consults to hide Runtime.evaluate / chrome.runtime
    /// tells. Off by default so non-automation use keeps full CDP behavior.
    #[serde(default)]
    pub strip_automation_tells: bool,
}

const fn default_version() -> u32 {
    1
}
