use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct ValidationTarget {
    pub name: String,
    pub url: String,
    pub wait_ms: u64,
    pub extractor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Verdict {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub target: String,
    pub verdict: Verdict,
    pub detail: String,
    pub raw: String,
    pub duration_ms: u64,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "PASS"),
            Verdict::Warn => write!(f, "WARN"),
            Verdict::Fail => write!(f, "FAIL"),
        }
    }
}
