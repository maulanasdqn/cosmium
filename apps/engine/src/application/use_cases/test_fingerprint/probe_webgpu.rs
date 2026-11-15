use super::probe::{ProbeDef, simple};
use crate::domain::profile::Profile;

const ADAPTER_INFO: &str = "(await navigator.gpu.requestAdapter()).info";

pub(super) fn webgpu_probes(p: &Profile) -> Vec<ProbeDef> {
    let Some(adapter) = &p.gpu.webgpu_adapter else {
        return Vec::new();
    };
    let mut probes = vec![
        simple(
            "webgpu_vendor",
            &format!("{ADAPTER_INFO}.vendor"),
            &adapter.vendor,
        ),
        simple(
            "webgpu_architecture",
            &format!("{ADAPTER_INFO}.architecture"),
            &adapter.architecture,
        ),
    ];
    if let (Some(min), Some(max)) = (adapter.subgroup_min_size, adapter.subgroup_max_size) {
        probes.push(simple(
            "webgpu_subgroups",
            &format!("(i=>i.subgroupMinSize+','+i.subgroupMaxSize)({ADAPTER_INFO})"),
            &format!("{min},{max}"),
        ));
    }
    probes
}
