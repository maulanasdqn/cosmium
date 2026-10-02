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
        wait_ms: 0,
        extractor: include_str!("extractors/creepjs.js").into(),
    }
}

fn pixelscan() -> ValidationTarget {
    ValidationTarget {
        name: "pixelscan".into(),
        url: "https://pixelscan.net/fingerprint-check".into(),
        wait_ms: 0,
        extractor: include_str!("extractors/pixelscan.js").into(),
    }
}

fn browserleaks() -> ValidationTarget {
    ValidationTarget {
        name: "browserleaks".into(),
        url: "https://browserleaks.com/javascript".into(),
        wait_ms: 0,
        extractor: include_str!("extractors/browserleaks.js").into(),
    }
}

fn bot_check(url: &str) -> ValidationTarget {
    ValidationTarget {
        name: "botcheck".into(),
        url: url.to_owned(),
        wait_ms: 0,
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
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let get = |k: &str| v.get(k).and_then(serde_json::Value::as_u64);
    let (Some(headless), Some(stealth), Some(like)) =
        (get("headless"), get("stealth"), get("like_headless"))
    else {
        return (
            Verdict::Warn,
            format!(
                "could not parse result: {}",
                crate::domain::text::prefix(raw, 100)
            ),
        );
    };
    let detail = format!("headless {headless}%, stealth {stealth}%, like-headless {like}%");
    let verdict = if headless > 0 || stealth > 0 {
        Verdict::Fail
    } else if like >= 50 {
        Verdict::Warn
    } else {
        Verdict::Pass
    };
    (verdict, detail)
}

fn evaluate_pixelscan(raw: &str) -> (Verdict, String) {
    if raw.starts_with("TIMEOUT") {
        return (Verdict::Warn, "page did not load in time".into());
    }
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let field = |k: &str| {
        v.get(k)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("?")
            .to_owned()
    };
    let verdict = field("verdict");
    let detail = format!(
        "{verdict}: {} / {} / {}",
        field("location"),
        field("fingerprint"),
        field("bot")
    );
    match verdict.as_str() {
        "consistent" => (Verdict::Pass, detail),
        "inconsistent" => (Verdict::Fail, detail),
        _ => (Verdict::Warn, detail),
    }
}

fn evaluate_browserleaks(raw: &str) -> (Verdict, String) {
    if raw.is_empty() || raw == "{}" {
        return (Verdict::Warn, "no data extracted".into());
    }
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let count = v.as_object().map_or(0, serde_json::Map::len);
    if count > 5 {
        (Verdict::Pass, format!("{count} properties extracted"))
    } else {
        (Verdict::Warn, format!("only {count} properties"))
    }
}

fn evaluate_botcheck(raw: &str) -> (Verdict, String) {
    let v: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
    let body = v
        .get("body")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
        .to_lowercase();
    let url = v
        .get("url")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
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

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
