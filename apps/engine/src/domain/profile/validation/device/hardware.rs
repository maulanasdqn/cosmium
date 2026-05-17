use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

pub fn hardware_concurrency(p: &Profile) -> Vec<Diagnostic> {
    let n = p.hardware.hardware_concurrency;
    let mut out = Vec::new();
    if n < 2 {
        out.push(Diagnostic::err(
            "hardware.hardware_concurrency",
            "must be >= 2; single-core devices barely exist on real Chrome",
        ));
    }
    if n % 2 != 0 {
        out.push(Diagnostic::warn(
            "hardware.hardware_concurrency",
            format!("{n} is odd — most real CPUs report even core counts"),
        ));
    }
    out
}

pub fn device_memory(p: &Profile) -> Vec<Diagnostic> {
    let m = p.hardware.device_memory_gb;
    let allowed = [0.25_f32, 0.5, 1.0, 2.0, 4.0, 8.0];
    if !allowed.iter().any(|a| (a - m).abs() < f32::EPSILON) {
        vec![Diagnostic::err(
            "hardware.device_memory_gb",
            format!("Chrome rounds deviceMemory to {allowed:?}; got {m}"),
        )]
    } else {
        vec![]
    }
}

pub fn ram_cores_plausible(p: &Profile) -> Vec<Diagnostic> {
    let cores = p.hardware.hardware_concurrency as f32;
    let ram = p.hardware.device_memory_gb;
    if cores >= 8.0 && ram <= 1.0 {
        return vec![Diagnostic::warn(
            "hardware",
            "implausible combo: many cores with very little RAM looks synthetic",
        )];
    }
    if cores <= 2.0 && ram >= 8.0 {
        return vec![Diagnostic::warn(
            "hardware",
            "implausible combo: few cores with lots of RAM looks synthetic",
        )];
    }
    vec![]
}
