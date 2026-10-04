use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredResult {
    pub id: String,
    pub created_at_ms: u64,
    #[serde(default)]
    pub title: Option<String>,
    pub payload: Value,
    pub result: Value,
    #[serde(default)]
    pub ai_request: Option<Value>,
    #[serde(default)]
    pub ai: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSummary {
    pub id: String,
    pub created_at_ms: u64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub final_url: Option<String>,
    pub profile: Option<String>,
    pub http_status: Option<i64>,
    pub blocked: bool,
    pub elapsed_ms: Option<u64>,
    pub has_ai: bool,
}

fn text(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

impl StoredResult {
    pub fn summary(&self) -> ResultSummary {
        ResultSummary {
            id: self.id.clone(),
            created_at_ms: self.created_at_ms,
            title: self.title.clone(),
            url: text(&self.payload, "url"),
            final_url: text(&self.result, "final_url"),
            profile: text(&self.payload, "profile"),
            http_status: self.result.get("http_status").and_then(Value::as_i64),
            blocked: self
                .result
                .get("blocked")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            elapsed_ms: self.result.get("elapsed_ms").and_then(Value::as_u64),
            has_ai: self.ai.is_some(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StoredResult;

    #[test]
    fn summary_reads_payload_and_result_fields() {
        let stored = StoredResult {
            id: "id".into(),
            created_at_ms: 5,
            title: Some("Shop".into()),
            payload: serde_json::json!({"url": "https://a.test", "profile": "p"}),
            result: serde_json::json!({"final_url": "https://a.test/", "http_status": 200, "blocked": false, "elapsed_ms": 900}),
            ai_request: None,
            ai: None,
        };
        let s = stored.summary();
        assert_eq!(s.url.as_deref(), Some("https://a.test"));
        assert_eq!(s.http_status, Some(200));
        assert_eq!(s.elapsed_ms, Some(900));
        assert!(!s.blocked && !s.has_ai);
    }
}
