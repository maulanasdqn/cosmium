use crate::domain::profile::Profile;

use super::switches::primary_lang;

pub fn profile_to_env(p: &Profile) -> Vec<(String, String)> {
    let locale_utf8 = bcp47_to_posix_utf8(primary_lang(&p.locale.languages));
    vec![
        ("TZ".into(), p.locale.timezone.clone()),
        ("LANG".into(), locale_utf8.clone()),
        ("LC_ALL".into(), locale_utf8),
    ]
}

fn bcp47_to_posix_utf8(tag: &str) -> String {
    let mut parts = tag.split('-');
    let base = parts.next().unwrap_or("en").to_ascii_lowercase();
    let region = parts
        .next()
        .map(|r| r.to_ascii_uppercase())
        .unwrap_or_else(|| base.to_ascii_uppercase());
    format!("{base}_{region}.UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mac_fixture() -> Profile {
        let raw = include_str!("../../../../../../profiles/macos_m2_en-us.json");
        serde_json::from_str(raw).expect("fixture parses")
    }

    #[test]
    fn env_emits_tz_and_locale_for_mac() {
        let env = profile_to_env(&mac_fixture());
        assert!(env.contains(&("TZ".into(), "America/Los_Angeles".into())));
        assert!(env.contains(&("LANG".into(), "en_US.UTF-8".into())));
        assert!(env.contains(&("LC_ALL".into(), "en_US.UTF-8".into())));
    }

    #[test]
    fn bcp47_two_letter_falls_back() {
        assert_eq!(bcp47_to_posix_utf8("en"), "en_EN.UTF-8");
        assert_eq!(bcp47_to_posix_utf8("fr-FR"), "fr_FR.UTF-8");
        assert_eq!(bcp47_to_posix_utf8("zh-Hant-TW"), "zh_HANT.UTF-8");
    }
}
