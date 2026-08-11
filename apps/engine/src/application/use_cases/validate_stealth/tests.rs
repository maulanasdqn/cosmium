use super::*;
use crate::domain::scraping::validation::Verdict;

#[test]
fn creepjs_high_score_passes() {
    let (v, _) = evaluate("creepjs", "Trust Score: 85%");
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn creepjs_low_score_fails() {
    let (v, _) = evaluate("creepjs", "Trust Score: 30%");
    assert_eq!(v, Verdict::Fail);
}

#[test]
fn creepjs_mid_score_warns() {
    let (v, _) = evaluate("creepjs", "Trust Score: 55%");
    assert_eq!(v, Verdict::Warn);
}

#[test]
fn creepjs_timeout_warns() {
    let (v, _) = evaluate("creepjs", "TIMEOUT:still loading");
    assert_eq!(v, Verdict::Warn);
}

#[test]
fn pixelscan_consistent_passes() {
    let (v, _) = evaluate("pixelscan", "Browser Fingerprint: Consistent");
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn pixelscan_inconsistent_fails() {
    let (v, _) = evaluate("pixelscan", "Inconsistent fingerprint detected");
    assert_eq!(v, Verdict::Fail);
}

#[test]
fn browserleaks_many_props_passes() {
    let raw = r#"{"UA":"Chrome","Lang":"en","TZ":"UTC","Mem":"8","Cores":"4","Screen":"1920","PDF":"true"}"#;
    let (v, _) = evaluate("browserleaks", raw);
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn browserleaks_empty_warns() {
    let (v, _) = evaluate("browserleaks", "{}");
    assert_eq!(v, Verdict::Warn);
}

#[test]
fn botcheck_clean_passes() {
    let raw = r#"{"url":"https://example.com/shop","title":"Shop","body":"Welcome to our store"}"#;
    let (v, _) = evaluate("botcheck", raw);
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn botcheck_captcha_url_fails() {
    let raw = r#"{"url":"https://example.com/captcha","title":"Verify","body":"Please verify"}"#;
    let (v, _) = evaluate("botcheck", raw);
    assert_eq!(v, Verdict::Fail);
}

#[test]
fn botcheck_access_denied_fails() {
    let raw = r#"{"url":"https://example.com","title":"Blocked","body":"Access Denied - unusual traffic"}"#;
    let (v, _) = evaluate("botcheck", raw);
    assert_eq!(v, Verdict::Fail);
}

#[test]
fn builtin_targets_default_three() {
    let targets = builtin_targets(None);
    assert_eq!(targets.len(), 3);
    assert_eq!(targets[0].name, "creepjs");
    assert_eq!(targets[1].name, "pixelscan");
    assert_eq!(targets[2].name, "browserleaks");
}

#[test]
fn builtin_targets_adds_botcheck() {
    let targets = builtin_targets(Some("https://traveloka.com"));
    assert_eq!(targets.len(), 4);
    assert_eq!(targets[3].name, "botcheck");
    assert_eq!(targets[3].url, "https://traveloka.com");
}
