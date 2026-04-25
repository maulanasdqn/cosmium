use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gpu {
    pub vendor: String,
    pub renderer: String,
    pub vendor_id: String,
    pub device_id: String,
    #[serde(default = "default_webgl_version")]
    pub webgl_version: String,
    #[serde(default)]
    pub webgpu_adapter: Option<WebGpuAdapter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGpuAdapter {
    pub vendor: String,
    pub architecture: String,
    pub device: String,
}

fn default_webgl_version() -> String {
    "WebGL 2.0 (OpenGL ES 3.0 Chromium)".to_owned()
}
