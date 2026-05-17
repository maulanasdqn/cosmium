use std::sync::Arc;

use anyhow::{Context, Result};

use crate::domain::llm::{ChatMessage, ChatRequest, LlmClient};
use crate::domain::profile::{Diagnostic, Profile, validation};

const SYSTEM_PROMPT: &str = "You repair cosmium fingerprint profiles.\n\
You receive a profile JSON and a list of coherence diagnostics. Output a corrected profile JSON\n\
that resolves every Error diagnostic and preserves every other field unchanged.\n\
Output strict JSON only — no prose, no markdown fences.";

pub struct RepairProfile {
    llm: Arc<dyn LlmClient>,
    model: String,
}

pub struct RepairProfileInput {
    pub profile: Profile,
    pub diagnostics: Vec<Diagnostic>,
}

pub struct RepairProfileOutput {
    pub profile: Profile,
    pub diagnostics: Vec<Diagnostic>,
    pub raw: String,
}

impl RepairProfile {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn execute(&self, input: RepairProfileInput) -> Result<RepairProfileOutput> {
        let profile_json = serde_json::to_string_pretty(&input.profile)?;
        let diag_lines: String = input
            .diagnostics
            .iter()
            .map(|d| format!("- [{:?}] {}: {}", d.severity, d.field, d.message))
            .collect::<Vec<_>>()
            .join("\n");
        let user = format!("Profile:\n{profile_json}\n\nDiagnostics:\n{diag_lines}");
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(SYSTEM_PROMPT), ChatMessage::user(user)],
            temperature: Some(0.0),
            json_mode: true,
        };
        let resp = self.llm.chat(req).await.context("openrouter chat call")?;
        let profile: Profile = serde_json::from_str(&resp.content).with_context(|| {
            format!(
                "parsing LLM JSON: {}",
                &resp.content[..resp.content.len().min(400)]
            )
        })?;
        let diagnostics = validation::validate(&profile);
        Ok(RepairProfileOutput {
            profile,
            diagnostics,
            raw: resp.content,
        })
    }
}
