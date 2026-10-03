use crate::domain::profile::{Profile, Severity, validation};
use crate::domain::runtime::user_agent_for;

use super::dto_platform::ProfileSummary;

fn ua_major(ua: &str) -> Option<String> {
    let rest = ua.split("Chrome/").nth(1)?;
    let major: String = rest.chars().take_while(char::is_ascii_digit).collect();
    (!major.is_empty()).then_some(major)
}

pub fn summarize(name: &str, p: &Profile) -> ProfileSummary {
    let diags = validation::validate(p);
    let errors = diags
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .count();
    let user_agent = user_agent_for(p);
    ProfileSummary {
        name: name.to_owned(),
        navigator_platform: Some(p.identity.navigator_platform.clone()),
        client_hints_platform: Some(p.identity.client_hints.platform.clone()),
        chrome_version: p.chrome_version.clone().or_else(|| ua_major(&user_agent)),
        user_agent: Some(user_agent),
        gpu_renderer: Some(p.gpu.renderer.clone()),
        timezone: Some(p.locale.timezone.clone()),
        languages: Some(p.locale.languages.clone()),
        screen: Some(format!("{}x{}", p.screen.width, p.screen.height)),
        device_pixel_ratio: Some(p.screen.device_pixel_ratio),
        errors: Some(errors),
        warnings: Some(diags.len() - errors),
        load_error: None,
    }
}

pub fn failed(name: &str, error: impl std::fmt::Display) -> ProfileSummary {
    ProfileSummary {
        name: name.to_owned(),
        load_error: Some(error.to_string()),
        ..ProfileSummary::default()
    }
}

#[cfg(test)]
mod tests {
    use super::{failed, summarize, ua_major};
    use crate::domain::profile::Profile;

    fn fixture() -> Profile {
        let raw = include_str!("../../../../../profiles/win11_rtx3060_en-us.json");
        serde_json::from_str(raw).unwrap_or_else(|e| panic!("fixture parses: {e}"))
    }

    #[test]
    fn summarizes_reference_profile() {
        let s = summarize("win11", &fixture());
        assert_eq!(s.navigator_platform.as_deref(), Some("Win32"));
        assert_eq!(s.client_hints_platform.as_deref(), Some("Windows"));
        assert_eq!(s.chrome_version.as_deref(), Some("154.0.8037.57"));
        assert_eq!(s.screen.as_deref(), Some("1920x1080"));
        assert_eq!(s.errors, Some(0));
        assert!(
            s.user_agent
                .is_some_and(|ua| ua.contains("Chrome/154.0.0.0"))
        );
        assert!(s.load_error.is_none());
    }

    #[test]
    fn failed_summary_keeps_only_name_and_error() {
        let s = failed("broken", "bad json");
        assert_eq!(s.load_error.as_deref(), Some("bad json"));
        assert!(s.navigator_platform.is_none() && s.errors.is_none());
    }

    #[test]
    fn extracts_major_from_user_agent() {
        assert_eq!(
            ua_major("Mozilla/5.0 Chrome/154.0.0.0 Safari").as_deref(),
            Some("154")
        );
        assert_eq!(ua_major("Mozilla/5.0 Firefox/130"), None);
    }
}
