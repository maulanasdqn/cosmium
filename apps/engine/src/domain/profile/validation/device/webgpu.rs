use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

pub(crate) fn webgpu_matches_webgl(p: &Profile) -> Vec<Diagnostic> {
    let Some(adapter) = &p.gpu.webgpu_adapter else {
        return vec![];
    };
    let mut out = Vec::new();
    let webgl = format!("{} {}", p.gpu.vendor, p.gpu.renderer).to_lowercase();
    if !webgl.contains(&adapter.vendor.to_lowercase()) {
        out.push(Diagnostic::err(
            "gpu.webgpu_adapter.vendor",
            format!(
                "WebGPU vendor '{}' does not appear in the WebGL vendor or renderer; sites compare the two",
                adapter.vendor
            ),
        ));
    }
    match (adapter.subgroup_min_size, adapter.subgroup_max_size) {
        (Some(min), Some(max)) if min == 0 || min > max => out.push(Diagnostic::err(
            "gpu.webgpu_adapter.subgroup_min_size",
            "subgroup_min_size must be non-zero and not larger than subgroup_max_size",
        )),
        (Some(_), Some(_)) => {}
        _ => out.push(Diagnostic::warn(
            "gpu.webgpu_adapter",
            "without subgroup_min_size and subgroup_max_size the real GPU's subgroup sizes leak through adapter.info",
        )),
    }
    out
}

#[cfg(test)]
mod tests {
    use super::webgpu_matches_webgl;
    use crate::domain::profile::{Profile, WebGpuAdapter};

    fn with_adapter(vendor: &str, min: Option<u32>, max: Option<u32>) -> Profile {
        let raw = include_str!("../../../../../profiles/win11_rtx3060_en-us.json");
        let mut p: Profile = serde_json::from_str(raw).expect("fixture parses");
        p.gpu.webgpu_adapter = Some(WebGpuAdapter {
            vendor: vendor.to_owned(),
            architecture: "ampere".to_owned(),
            device: String::new(),
            subgroup_min_size: min,
            subgroup_max_size: max,
        });
        p
    }

    #[test]
    fn matching_vendor_with_subgroups_is_clean() {
        assert!(webgpu_matches_webgl(&with_adapter("nvidia", Some(32), Some(32))).is_empty());
    }

    #[test]
    fn flags_vendor_that_contradicts_webgl() {
        let diags = webgpu_matches_webgl(&with_adapter("amd", Some(64), Some(64)));
        assert!(diags.iter().any(|d| d.field == "gpu.webgpu_adapter.vendor"));
    }

    #[test]
    fn flags_inverted_or_missing_subgroups() {
        let inverted = webgpu_matches_webgl(&with_adapter("nvidia", Some(64), Some(32)));
        assert!(
            inverted
                .iter()
                .any(|d| d.field == "gpu.webgpu_adapter.subgroup_min_size")
        );
        let missing = webgpu_matches_webgl(&with_adapter("nvidia", None, None));
        assert!(missing.iter().any(|d| d.field == "gpu.webgpu_adapter"));
    }
}
