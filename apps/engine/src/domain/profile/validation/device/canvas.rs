use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

pub fn canvas_noise_seed(p: &Profile) -> Vec<Diagnostic> {
    let seed = &p.canvas_noise.seed;
    if seed.len() != 32 || !seed.chars().all(|c| c.is_ascii_hexdigit()) {
        vec![Diagnostic::err(
            "canvas_noise.seed",
            "must be exactly 32 hex characters (16-byte seed)",
        )]
    } else {
        vec![]
    }
}
