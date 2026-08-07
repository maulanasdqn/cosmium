use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screen {
    pub width: u32,
    pub height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
    pub device_pixel_ratio: f32,
    #[serde(default)]
    pub avail_left: u32,
    #[serde(default)]
    pub avail_top: u32,
}
