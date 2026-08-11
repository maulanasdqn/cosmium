use crate::domain::scraping::validation::{ValidationTarget, Verdict};

pub fn builtin_targets(bot_check_url: Option<&str>) -> Vec<ValidationTarget> {
    let mut out = vec![creepjs(), pixelscan(), browserleaks()];
    if let Some(url) = bot_check_url {
        out.push(bot_check(url));
    }
    out
}

fn creepjs() -> ValidationTarget {
    ValidationTarget {
        name: "creepjs".into(),
        url: "https://abrahamjuliot.github.io/creepjs/".into(),
        wait_ms: 25000,
        extractor: include_str!("extractors/creepjs.js").into(),
    }
}

fn pixelscan() -> ValidationTarget {
    ValidationTarget {
        name: "pixelscan".into(),
        url: "https://pixelscan.net/".into(),
        wait_ms: 10000,
        extractor: include_str!("extractors/pixelscan.js").into(),
    }
}

fn browserleaks() -> ValidationTarget {
    ValidationTarget {
        name: "browserleaks".into(),
        url: "https://browserleaks.com/javascript".into(),
        wait_ms: 8000,
        extractor: include_str!("extractors/browserleaks.js").into(),
    }
}

fn bot_check(url: &str) -> ValidationTarget {
    ValidationTarget {
        name: "botcheck".into(),
        url: url.to_owned(),
        wait_ms: 15000,
        extractor: include_str!("extractors/botcheck.js").into(),
    }
}

pub fn evaluate(target_name: &str, raw: &str) -> (Verdict, String) {
    match target_name {
        "creepjs" => evaluate_creepjs(raw),
        "pixelscan" => evaluate_pixelscan(raw),
        "browserleaks" => evaluate_browserleaks(raw),
        "botcheck" => evaluate_botcheck(raw),
        _ => (Verdict::Warn, "unknown target".into()),
    }
}

fn evaluate_creepjs(raw: &str) -> (Verdict, String) {
    if raw.starts_with("TIMEOUT") {
        return (Verdict::Warn, "page did not load in time".into());
    }
    let lower = raw.to_lowercase();
    if lower.contains("trust score") || lower.contains('%') {
        if let Some(pct) = extract_percentage(&lower) {
            if pct >= 70.0 {
                return (Verdict::Pass, format!("trust score {pct:.0}%"));
            }
            if pct >= 40.0 {
                return (Verdict::Warn, format!("trust score {pct:.0}%"));
            }
            return (Verdict::Fail, format!("trust score {pct:.0}%"));
        }
    }
    (
        Verdict::Warn,
        format!("could not parse score: {}", &raw[..raw.len().min(100)]),
    )
}

fn evaluate_pixelscan(raw: &str) -> (Verdict, String) {
    if raw.starts_with("TIMEOUT") {
        return (Verdict::Warn, "page did not load in time".into());
    }
    let lower = raw.to_lowercase();
    if lower.contains("consistent") && !lower.contains("inconsistent") {
        return (Verdict::Pass, "consistent fingerprint".into());
    }
    if lower.contains("inconsistent") {
        return (Verdict::Fail, "inconsistent fingerprint detected".into());
    }
    (
        Verdict::Warn,
        format!("unclear result: {}", &raw[..raw.len().min(100)]),
    )
}

fn evaluate_browserleaks(raw: &str) -> (Verdict, String) {
    if raw.is_empty() || raw == "{}" {
        return (Verdict::Warn, "no data extracted".into());
    }
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let count = v.as_object().map_or(0, |o| o.len());
    if count > 5 {
        (Verdict::Pass, format!("{count} properties extracted"))
    } else {
        (Verdict::Warn, format!("only {count} properties"))
    }
}

fn evaluate_botcheck(raw: &str) -> (Verdict, String) {
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let body = v["body"].as_str().unwrap_or("").to_lowercase();
    let url = v["url"].as_str().unwrap_or("");
    let blocked = [
        "captcha",
        "access denied",
        "just a moment",
        "are you a robot",
        "unusual traffic",
        "cf-browser-verification",
    ];
    for m in &blocked {
        if body.contains(m) {
            return (Verdict::Fail, format!("blocked: body contains '{m}'"));
        }
    }
    let walls = ["captcha", "/challenge", "/blocked", "verify/bot"];
    let lower_url = url.to_lowercase();
    for m in &walls {
        if lower_url.contains(m) {
            return (Verdict::Fail, format!("redirected to wall: {url}"));
        }
    }
    (Verdict::Pass, "page loaded without block".into())
}

fn extract_percentage(text: &str) -> Option<f64> {
    let re = regex::Regex::new(r"(\d+(?:\.\d+)?)%").ok()?;
    re.captures(text)?.get(1)?.as_str().parse().ok()
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
