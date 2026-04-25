use anyhow::{Result, bail};
use serde::Deserialize;

use super::probe::{ProbeDef, ProbeResult};

#[derive(Deserialize)]
struct Raw {
    id: String,
    value: Option<String>,
    error: Option<String>,
}

pub fn extract(dom: &str, probes: &[ProbeDef]) -> Result<Vec<ProbeResult>> {
    let raw_lines = extract_pre_lines(dom);
    if raw_lines.is_empty() {
        bail!("no probe results captured in DOM");
    }
    let mut by_id: std::collections::HashMap<String, Raw> = std::collections::HashMap::new();
    for line in raw_lines {
        if let Ok(r) = serde_json::from_str::<Raw>(&line) {
            by_id.insert(r.id.clone(), r);
        }
    }
    let mut results = Vec::with_capacity(probes.len());
    for p in probes {
        let entry = by_id.get(p.id);
        match entry {
            Some(Raw {
                error: Some(e), ..
            }) => results.push(ProbeResult {
                id: p.id.to_owned(),
                passed: false,
                got: String::new(),
                expected: p.expected_display.clone(),
                error: Some(e.clone()),
            }),
            Some(Raw {
                value: Some(v), ..
            }) => {
                let passed = p.expected.is_match(v);
                results.push(ProbeResult {
                    id: p.id.to_owned(),
                    passed,
                    got: v.clone(),
                    expected: p.expected_display.clone(),
                    error: None,
                });
            }
            _ => results.push(ProbeResult {
                id: p.id.to_owned(),
                passed: false,
                got: String::new(),
                expected: p.expected_display.clone(),
                error: Some("missing".into()),
            }),
        }
    }
    Ok(results)
}

fn extract_pre_lines(dom: &str) -> Vec<String> {
    let start = match dom.find(r#"<pre id="out">"#) {
        Some(i) => i + r#"<pre id="out">"#.len(),
        None => return vec![],
    };
    let end = match dom[start..].find("</pre>") {
        Some(i) => start + i,
        None => return vec![],
    };
    dom[start..end]
        .lines()
        .map(|l| l.trim().to_owned())
        .filter(|l| !l.is_empty())
        .collect()
}
