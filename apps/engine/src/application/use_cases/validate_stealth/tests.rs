use super::*;
use crate::domain::scraping::validation::Verdict;

#[test]
fn creepjs_clean_passes() {
    let (v, _) = evaluate(
        "creepjs",
        r#"{"headless":0,"stealth":0,"like_headless":31}"#,
    );
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn creepjs_detected_stealth_fails() {
    let (v, _) = evaluate(
        "creepjs",
        r#"{"headless":0,"stealth":40,"like_headless":10}"#,
    );
    assert_eq!(v, Verdict::Fail);
}

#[test]
fn creepjs_mostly_headless_like_warns() {
    let (v, _) = evaluate(
        "creepjs",
        r#"{"headless":0,"stealth":0,"like_headless":60}"#,
    );
    assert_eq!(v, Verdict::Warn);
}

#[test]
fn creepjs_timeout_warns() {
    let (v, _) = evaluate("creepjs", "TIMEOUT:still loading");
    assert_eq!(v, Verdict::Warn);
}

#[test]
fn pixelscan_consistent_passes() {
    let (v, _) = evaluate(
        "pixelscan",
        r#"{"verdict":"consistent","bot":"No automated behavior detected"}"#,
    );
    assert_eq!(v, Verdict::Pass);
}

#[test]
fn pixelscan_inconsistent_fails() {
    let (v, _) = evaluate(
        "pixelscan",
        r#"{"verdict":"inconsistent","location":"Timezone spoofed"}"#,
    );
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
    let targets = builtin_targets(Some("https://example.com"));
    assert_eq!(targets.len(), 4);
    assert_eq!(targets[3].name, "botcheck");
    assert_eq!(targets[3].url, "https://example.com");
}
