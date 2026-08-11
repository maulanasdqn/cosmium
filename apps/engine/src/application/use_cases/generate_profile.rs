use std::sync::Arc;

use anyhow::{Context, Result, bail};

use crate::domain::llm::{ChatMessage, ChatRequest, LlmClient};
use crate::domain::profile::{Diagnostic, Profile, validation};

const SCHEMA: &str = include_str!("../../../profiles/schema.json");
const REFERENCE: &str = include_str!("../../../profiles/win11_rtx3060_en-us.json");

const SYSTEM_PROMPT: &str = "You are a browser fingerprint engineer producing cosmium profiles.\n\
Output strict JSON only — no prose, no markdown fences, no commentary.\n\
\n\
Hard rules:\n\
- UA major Chrome version must equal client_hints.brands[brand=Google Chrome].version\n\
- navigator_platform must match client_hints.platform (Win32↔Windows, MacIntel↔macOS, Linux x86_64↔Linux)\n\
- locale.languages[0] must be the prefix of locale.accept_language\n\
- locale.timezone must be a real IANA zone matching the locale region\n\
- screen.color_depth must equal screen.pixel_depth\n\
- screen.avail_width <= screen.width and avail_height <= screen.height\n\
- hardware.hardware_concurrency must be even (real CPUs)\n\
- hardware.device_memory_gb must be one of [0.25, 0.5, 1, 2, 4, 8]\n\
- gpu.renderer must NOT contain \"SwiftShader\" or \"0x0000C0DE\"\n\
- gpu.vendor and gpu.renderer must agree on the GPU brand\n\
- canvas_noise.seed must be exactly 32 hex characters\n\
- media_devices must contain at least one audioinput, one audiooutput, one videoinput\n\
- voices must contain at least one entry whose lang prefix matches locale.languages[0]\n\
- fonts.installed must match the platform (Windows fonts for Win32, macOS fonts for MacIntel)";

pub struct GenerateProfile {
    llm: Arc<dyn LlmClient>,
    model: String,
}

pub struct GenerateProfileInput {
    pub persona: String,
    pub name: String,
}

pub struct GenerateProfileOutput {
    pub profile: Profile,
    pub diagnostics: Vec<Diagnostic>,
    pub raw: String,
}

impl GenerateProfile {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn execute(&self, input: GenerateProfileInput) -> Result<GenerateProfileOutput> {
        let user = format!(
            "Schema:\n{SCHEMA}\n\nReference profile:\n{REFERENCE}\n\nGenerate a profile named {:?} for this persona:\n{}",
            input.name, input.persona
        );
        let req = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(SYSTEM_PROMPT), ChatMessage::user(user)],
            temperature: Some(0.2),
            json_mode: true,
        };
        let resp = self.llm.chat(req).await.context("openrouter chat call")?;
        let mut profile: Profile = serde_json::from_str(&resp.content)
            .with_context(|| format!("parsing LLM JSON: {}", truncate(&resp.content, 400)))?;
        if profile.name.is_empty() {
            profile.name = input.name.clone();
        }
        let diagnostics = validation::validate(&profile);
        Ok(GenerateProfileOutput {
            profile,
            diagnostics,
            raw: resp.content,
        })
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_owned()
    } else {
        format!("{}…", &s[..n])
    }
}

pub fn ensure_no_errors(diagnostics: &[Diagnostic]) -> Result<()> {
    let errors = diagnostics
        .iter()
        .filter(|d| d.severity == crate::domain::profile::Severity::Error)
        .count();
    if errors > 0 {
        bail!("LLM produced profile with {errors} coherence error(s) — try repair");
    }
    Ok(())
}
