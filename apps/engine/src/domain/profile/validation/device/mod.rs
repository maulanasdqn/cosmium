mod canvas;
mod gpu;
mod hardware;
mod platform;
mod screen;
mod touch;

pub(super) use canvas::canvas_noise_seed;
pub(super) use gpu::{gpu_vendor_matches_platform, webgl_renderer};
pub(super) use hardware::{device_memory, hardware_concurrency, ram_cores_plausible};
pub(super) use screen::{pixel_depth, screen_dimensions};
pub(super) use touch::touch_points_match_form_factor;
