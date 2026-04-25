use std::sync::Arc;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::domain::llm::{ChatMessage, ChatRequest, LlmClient};
use crate::domain::profile::{Diagnostic, Profile, validation};

const SYSTEM_PROMPT: &str = "You produce coherent variations of cosmium fingerprint profiles.\n\
Given a reference profile and a count N, output N distinct profile variants.\n\
Each variant differs from the reference in some combination of: Chrome major version,\n\
GPU vendor/renderer, primary locale, screen resolution, hardware concurrency, device memory,\n\
timezone — while remaining internally coherent (UA matches ClientHints major version,\n\
languages match accept_language and timezone region, GPU vendor matches renderer brand,\n\
fonts and voices match the platform).\n\
\n\
Each variant's `name` field must be unique and descriptive (e.g. win10_intel_uhd_pt-br).\n\
\n\
Output JSON only, no prose, no fences:\n\
{\"variants\": [<profile1>, <profile2>, ...]}\n\
\n\
Same hard rules as profile generation: even hardware_concurrency, device_memory_gb in\n\
[0.25, 0.5, 1, 2, 4, 8], color_depth == pixel_depth, no SwiftShader in renderer,\n\
canvas_noise.seed exactly 32 hex chars, at least one audioinput/audiooutput/videoinput.";

pub struct MutateProfile {
    llm: Arc<dyn LlmClient>,
    model: String,
}

pub struct MutateProfileInput {
    pub reference: Profile,
    pub count: usize,
    pub hint: Option<String>,
}

pub struct MutateProfileOutput {
    pub variants: Vec<VariantOutput>,
}

pub struct VariantOutput {
    pub profile: Profile,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Deserialize)]
struct LlmEnvelope {
    variants: Vec<Profile>,
}

impl MutateProfile {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn execute(&self, input: MutateProfileInput) -> Result<MutateProfileOutput> {
        let reference_json = serde_json::to_string_pretty(&input.reference)?;
        let mut user = format!(
            "Reference profile:\n{reference_json}\n\nProduce {} distinct variants.",
            input.count
        );
        if let Some(h) = &input.hint {
            user.push_str("\n\nVariation hint: ");
            user.push_str(h);
        }
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(SYSTEM_PROMPT), ChatMessage::user(user)],
            temperature: Some(0.7),
            json_mode: true,
        };
        let resp = self.llm.chat(req).await.context("openrouter chat call")?;
        let envelope: LlmEnvelope = serde_json::from_str(&resp.content).with_context(|| {
            format!(
                "parsing variants JSON: {}",
                &resp.content[..resp.content.len().min(400)]
            )
        })?;
        let variants = envelope
            .variants
            .into_iter()
            .map(|p| {
                let diagnostics = validation::validate(&p);
                VariantOutput {
                    profile: p,
                    diagnostics,
                }
            })
            .collect();
        Ok(MutateProfileOutput { variants })
    }
}
