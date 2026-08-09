use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkflowStep {
    Delay {
        duration_ms: u32,
    },
    Script {
        name: String,
        code: String,
        #[serde(default = "default_script_timeout")]
        timeout_seconds: u32,
    },
    Click {
        selector: String,
    },
    Input {
        selector: String,
        text: String,
    },
    Scroll {
        #[serde(default)]
        infinite: bool,
        #[serde(default)]
        selector: Option<String>,
        #[serde(default = "default_scroll_times")]
        times: u32,
    },
    Extract {
        name: String,
        selector: String,
        #[serde(default)]
        attribute: Option<String>,
        #[serde(default)]
        limit: u32,
    },
    FollowUrls {
        name: String,
        selector: String,
        #[serde(default)]
        attribute: Option<String>,
        #[serde(default)]
        limit: u32,
        #[serde(default)]
        workflow: Vec<WorkflowStep>,
    },
}

fn default_script_timeout() -> u32 {
    30
}

fn default_scroll_times() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_click() {
        let step = WorkflowStep::Click {
            selector: ".btn".into(),
        };
        let json = serde_json::to_string(&step).unwrap();
        let back: WorkflowStep = serde_json::from_str(&json).unwrap();
        assert!(matches!(back, WorkflowStep::Click { selector } if selector == ".btn"));
    }

    #[test]
    fn round_trip_extract() {
        let json = r#"{"type":"extract","name":"prices","selector":".price","limit":10}"#;
        let step: WorkflowStep = serde_json::from_str(json).unwrap();
        assert!(
            matches!(step, WorkflowStep::Extract { name, limit, .. } if name == "prices" && limit == 10)
        );
    }

    #[test]
    fn round_trip_workflow_vec() {
        let json = r#"[
            {"type":"click","selector":".search"},
            {"type":"delay","duration_ms":1500},
            {"type":"extract","name":"results","selector":".item","limit":50}
        ]"#;
        let steps: Vec<WorkflowStep> = serde_json::from_str(json).unwrap();
        assert_eq!(steps.len(), 3);
    }
}
