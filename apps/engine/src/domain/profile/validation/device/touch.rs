use crate::domain::profile::Profile;
use crate::domain::profile::validation::Diagnostic;

pub fn touch_points_match_form_factor(p: &Profile) -> Vec<Diagnostic> {
    let ua = &p.identity.user_agent;
    let mobile_ua = ua.contains("Mobile") || ua.contains("Android") || ua.contains("iPhone");
    let touches = p.hardware.max_touch_points;
    if mobile_ua && touches == 0 {
        return vec![Diagnostic::err(
            "hardware.max_touch_points",
            "mobile UA reported but max_touch_points == 0 (mobile devices report >=1)",
        )];
    }
    if !mobile_ua && touches > 0 && touches < 5 {
        return vec![Diagnostic::warn(
            "hardware.max_touch_points",
            "desktop UA with 1-4 touch points is unusual; convertibles typically report 10",
        )];
    }
    vec![]
}
