use std::str::FromStr;

use chrono_tz::Tz;
use once_cell::sync::Lazy;
use regex::Regex;

use super::{Diagnostic, Profile};

pub(super) fn accept_language(p: &Profile) -> Vec<Diagnostic> {
    let Some(first) = p.locale.languages.first() else {
        return vec![Diagnostic::err(
            "locale.languages",
            "languages array is empty",
        )];
    };
    if !p.locale.accept_language.starts_with(first.as_str()) {
        return vec![Diagnostic::err(
            "locale.accept_language",
            format!(
                "must start with {first:?} (first entry of locale.languages), got {:?}",
                p.locale.accept_language
            ),
        )];
    }
    vec![]
}

static BCP47: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z]{2,3}(-[a-zA-Z]{2,4})?(-[a-zA-Z0-9]{2,8})?$").expect("compile regex")
});

pub(super) fn languages(p: &Profile) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    for lang in &p.locale.languages {
        if !BCP47.is_match(lang) {
            out.push(Diagnostic::err(
                "locale.languages",
                format!("{lang:?} is not a valid BCP-47 tag"),
            ));
        }
    }
    out
}

pub(super) fn timezone(p: &Profile) -> Vec<Diagnostic> {
    if Tz::from_str(&p.locale.timezone).is_err() {
        vec![Diagnostic::err(
            "locale.timezone",
            format!("{:?} is not a valid IANA timezone", p.locale.timezone),
        )]
    } else {
        vec![]
    }
}

pub(super) fn voices(p: &Profile) -> Vec<Diagnostic> {
    let Some(primary) = p.locale.languages.first() else {
        return vec![];
    };
    let primary_base = primary.split('-').next().unwrap_or(primary);
    let has_match = p
        .voices
        .iter()
        .any(|v| v.lang.starts_with(primary_base) || v.default);
    if !has_match {
        vec![Diagnostic::warn(
            "voices",
            format!(
                "no voice matches primary language {primary_base:?} — real systems usually ship one"
            ),
        )]
    } else {
        vec![]
    }
}

pub(super) fn voice_defaults(p: &Profile) -> Vec<Diagnostic> {
    let default_count = p.voices.iter().filter(|v| v.default).count();
    if p.voices.is_empty() {
        return vec![Diagnostic::warn(
            "voices",
            "voices array is empty — real systems have 10-50 voices",
        )];
    }
    if default_count == 0 {
        return vec![Diagnostic::err(
            "voices",
            "no voice has default=true — exactly one must be the default",
        )];
    }
    if default_count > 1 {
        return vec![Diagnostic::err(
            "voices",
            format!(
                "{default_count} voices have default=true — exactly one must be the default"
            ),
        )];
    }
    vec![]
}
