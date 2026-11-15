use indexmap::IndexMap;
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
    #[serde(default)]
    pub webgl_limits: IndexMap<String, String>,
    #[serde(default)]
    pub webgl_excluded_extensions: Vec<String>,
}

impl Gpu {
    pub fn webgl_limits_switch(&self) -> Option<String> {
        if self.webgl_limits.is_empty() {
            return None;
        }
        Some(
            self.webgl_limits
                .iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join(","),
        )
    }

    pub fn webgl_excluded_extensions_switch(&self) -> Option<String> {
        if self.webgl_excluded_extensions.is_empty() {
            return None;
        }
        Some(self.webgl_excluded_extensions.join(","))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGpuAdapter {
    pub vendor: String,
    pub architecture: String,
    #[serde(default)]
    pub device: String,
    #[serde(default)]
    pub subgroup_min_size: Option<u32>,
    #[serde(default)]
    pub subgroup_max_size: Option<u32>,
}

impl WebGpuAdapter {
    pub fn push_flags(&self, f: &mut Vec<String>) {
        f.push(format!("--cosmium-webgpu-vendor={}", self.vendor));
        f.push(format!(
            "--cosmium-webgpu-architecture={}",
            self.architecture
        ));
        if let (Some(min), Some(max)) = (self.subgroup_min_size, self.subgroup_max_size) {
            f.push(format!("--cosmium-webgpu-subgroup-sizes={min},{max}"));
        }
    }
}

fn default_webgl_version() -> String {
    "WebGL 2.0 (OpenGL ES 3.0 Chromium)".to_owned()
}

#[cfg(test)]
mod tests {
    use super::Gpu;

    fn gpu() -> Gpu {
        serde_json::from_str(
            r#"{"vendor":"v","renderer":"r","vendor_id":"0x1","device_id":"0x2",
                "webgl_limits":{"MAX_TEXTURE_SIZE":"16384","MAX_VIEWPORT_DIMS":"16384x16384"},
                "webgl_excluded_extensions":["WEBGL_compressed_texture_astc"]}"#,
        )
        .expect("fixture parses")
    }

    #[test]
    fn builds_switch_values_in_declaration_order() {
        let g = gpu();
        assert_eq!(
            g.webgl_limits_switch().as_deref(),
            Some("MAX_TEXTURE_SIZE=16384,MAX_VIEWPORT_DIMS=16384x16384")
        );
        assert_eq!(
            g.webgl_excluded_extensions_switch().as_deref(),
            Some("WEBGL_compressed_texture_astc")
        );
    }

    #[test]
    fn webgpu_adapter_emits_vendor_architecture_and_subgroups() {
        let a: super::WebGpuAdapter = serde_json::from_str(
            r#"{"vendor":"nvidia","architecture":"ampere","subgroup_min_size":32,"subgroup_max_size":32}"#,
        )
        .expect("adapter parses");
        let mut f = Vec::new();
        a.push_flags(&mut f);
        assert_eq!(
            f,
            [
                "--cosmium-webgpu-vendor=nvidia",
                "--cosmium-webgpu-architecture=ampere",
                "--cosmium-webgpu-subgroup-sizes=32,32",
            ]
        );
    }

    #[test]
    fn webgpu_adapter_skips_subgroups_when_incomplete() {
        let a: super::WebGpuAdapter = serde_json::from_str(
            r#"{"vendor":"amd","architecture":"rdna-2","subgroup_min_size":64}"#,
        )
        .expect("adapter parses");
        let mut f = Vec::new();
        a.push_flags(&mut f);
        assert_eq!(f.len(), 2);
    }

    #[test]
    fn omits_switches_when_unset() {
        let g: Gpu = serde_json::from_str(
            r#"{"vendor":"v","renderer":"r","vendor_id":"0x1","device_id":"0x2"}"#,
        )
        .expect("minimal fixture parses");
        assert!(g.webgl_limits_switch().is_none());
        assert!(g.webgl_excluded_extensions_switch().is_none());
    }
}
