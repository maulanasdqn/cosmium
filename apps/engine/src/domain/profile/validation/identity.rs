use once_cell::sync::Lazy;
use regex::Regex;

use super::{Diagnostic, Profile};
use crate::domain::profile::identity::ClientHints;

pub(super) fn platform(p: &Profile) -> Vec<Diagnostic> {
    let ua_os = detect_ua_os(&p.identity.user_agent);
    let nav = &p.identity.navigator_platform;
    let ch = &p.identity.client_hints.platform;

    let nav_os = match nav.as_str() {
        "Win32" | "Win64" => Some("Windows"),
        "MacIntel" => Some("macOS"),
        s if s.starts_with("Linux") => Some("Linux"),
        _ => None,
    };

    let mut out = Vec::new();
    if let (Some(ua), Some(nav)) = (ua_os, nav_os) {
        if ua != nav {
            out.push(Diagnostic::err(
                "identity.navigator_platform",
                format!("UA implies {ua} but navigator.platform implies {nav}"),
            ));
        }
    }
    if let Some(ua) = ua_os {
        if ua != ch.as_str() {
            out.push(Diagnostic::err(
                "identity.client_hints.platform",
                format!("UA implies {ua} but Sec-CH-UA-Platform is {ch:?}"),
            ));
        }
    }
    out
}

fn detect_ua_os(ua: &str) -> Option<&'static str> {
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

static CHROME_VERSION_IN_UA: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Chrome/(\d+)").expect("compile regex"));

pub(super) fn chrome_version(p: &Profile) -> Vec<Diagnostic> {
    let ua_major = CHROME_VERSION_IN_UA
        .captures(&p.identity.user_agent)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());

    let ch_major = chrome_brand_version(&p.identity.client_hints);

    match (ua_major, ch_major) {
        (Some(ua), Some(ch)) if ua != ch => vec![Diagnostic::err(
            "identity.client_hints.brands",
            format!("UA reports Chrome/{ua} but ClientHints brand reports Chrome/{ch}"),
        )],
        (None, _) => vec![Diagnostic::warn(
            "identity.user_agent",
            "no Chrome major version found in user_agent",
        )],
        _ => vec![],
    }
}

fn chrome_brand_version(ch: &ClientHints) -> Option<u32> {
    ch.brands
        .iter()
        .find(|b| b.brand == "Google Chrome" || b.brand == "Chromium")
        .and_then(|b| b.version.parse().ok())
}

pub(super) fn brands(p: &Profile) -> Vec<Diagnostic> {
    let has_chrome = p
        .identity
        .client_hints
        .brands
        .iter()
        .any(|b| b.brand == "Google Chrome");
    if !has_chrome {
        vec![Diagnostic::warn(
            "identity.client_hints.brands",
            "no \"Google Chrome\" entry — real Chrome always emits one",
        )]
    } else {
        vec![]
    }
}
