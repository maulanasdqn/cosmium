use std::collections::HashSet;

use super::{Diagnostic, Profile};
use crate::domain::profile::identity::Identity;

pub(super) fn platform_staples_present(p: &Profile) -> Vec<Diagnostic> {
    let Some(os) = platform_of(&p.identity) else {
        return vec![];
    };
    let installed: HashSet<&str> = p.fonts.installed.iter().map(String::as_str).collect();

    let staples: &[&str] = match os {
        "macOS" => &[
            "Helvetica",
            "Helvetica Neue",
            "San Francisco",
            "Apple Color Emoji",
            "Lucida Grande",
            "Menlo",
        ],
        "Windows" => &[
            "Segoe UI",
            "Segoe UI Emoji",
            "Calibri",
            "Cambria",
            "Consolas",
            "Tahoma",
        ],
        "Linux" => &["DejaVu Sans", "Liberation Sans", "Noto Sans", "Ubuntu"],
        _ => return vec![],
    };

    let hits = staples.iter().filter(|s| installed.contains(*s)).count();
    if hits == 0 {
        return vec![Diagnostic::err(
            "fonts.installed",
            format!("{os} profile lists none of the platform staples ({staples:?})"),
        )];
    }
    if hits < staples.len() / 2 {
        return vec![Diagnostic::warn(
            "fonts.installed",
            format!(
                "{os} profile only contains {hits}/{} platform staples",
                staples.len()
            ),
        )];
    }
    vec![]
}

pub(super) fn no_cross_platform_leak(p: &Profile) -> Vec<Diagnostic> {
    let Some(os) = platform_of(&p.identity) else {
        return vec![];
    };
    let installed: HashSet<&str> = p.fonts.installed.iter().map(String::as_str).collect();

    let forbidden: &[(&str, &str)] = match os {
        "macOS" => &[
            ("Segoe UI", "Windows-only"),
            ("Calibri", "Windows-only"),
            ("Liberation Sans", "Linux distro font"),
            ("DejaVu Sans", "Linux distro font"),
            ("Noto Color Emoji", "Linux/Android emoji"),
        ],
        "Windows" => &[
            ("San Francisco", "macOS-only"),
            ("Apple Color Emoji", "macOS-only"),
            ("Helvetica Neue", "macOS-only"),
            ("DejaVu Sans", "Linux distro font"),
            ("Liberation Sans", "Linux distro font"),
        ],
        "Linux" => &[
            ("Segoe UI", "Windows-only"),
            ("San Francisco", "macOS-only"),
            ("Apple Color Emoji", "macOS-only"),
        ],
        _ => return vec![],
    };

    forbidden
        .iter()
        .filter(|(font, _)| installed.contains(*font))
        .map(|(font, why)| {
            Diagnostic::err(
                "fonts.installed",
                format!("{os} profile contains {font:?} ({why}) — fingerprint leak"),
            )
        })
        .collect()
}

fn platform_of(id: &Identity) -> Option<&'static str> {
    let ua = &id.user_agent;
    if ua.contains("Windows NT") {
        Some("Windows")
    } else if ua.contains("Mac OS X") || ua.contains("Macintosh") {
        Some("macOS")
    } else if ua.contains("Linux") || ua.contains("X11") {
        Some("Linux")
    } else {
        None
    }
}
