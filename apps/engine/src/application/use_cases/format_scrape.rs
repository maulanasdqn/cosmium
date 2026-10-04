use std::sync::Arc;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::domain::llm::{ChatMessage, ChatRequest, LlmClient};
use crate::domain::text::prefix;

const MAX_CONTENT_BYTES: usize = 60_000;

const SYSTEM_PROMPT: &str = "You turn content scraped from a web page into clean, well-structured JSON.\n\
Use short snake_case keys.\n\
When the page lists similar things (products, articles, search results, jobs, comments), return an object\n\
with an \"items\" array where every item has the same keys, plus any useful page-level fields.\n\
Keep values faithful to the source. Never invent, guess, or translate data. Use numbers for numeric values.\n\
Drop navigation, boilerplate, cookie banners, and duplicates.\n\
If the user gives an instruction, follow it for what to extract and how to shape it.\n\
Output strict JSON only, with no prose and no markdown fences.";

const DEFAULT_INSTRUCTION: &str = "Extract the main structured data from this page.";

pub struct FormatScrape {
    llm: Arc<dyn LlmClient>,
    model: String,
}

pub struct FormatScrapeInput {
    pub url: String,
    pub instruction: Option<String>,
    pub content: Value,
}

pub struct FormatScrapeOutput {
    pub data: Value,
    pub model: String,
}

impl FormatScrape {
    pub fn new(llm: Arc<dyn LlmClient>, model: String) -> Self {
        Self { llm, model }
    }

    pub async fn execute(&self, input: FormatScrapeInput) -> Result<FormatScrapeOutput> {
        let content = serde_json::to_string(&input.content)?;
        let instruction = input
            .instruction
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
            .unwrap_or(DEFAULT_INSTRUCTION);
        let user = format!(
            "Page: {}\nInstruction: {instruction}\n\nScraped content (JSON):\n{}",
            input.url,
            prefix(&content, MAX_CONTENT_BYTES)
        );
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![ChatMessage::system(SYSTEM_PROMPT), ChatMessage::user(user)],
            temperature: Some(0.2),
            json_mode: true,
        };
        let response = self.llm.chat(request).await.context("LLM chat call")?;
        let data = parse_json(&response.content)
            .with_context(|| format!("parsing LLM JSON: {}", prefix(&response.content, 300)))?;
        Ok(FormatScrapeOutput {
            data,
            model: response.model,
        })
    }
}

fn parse_json(raw: &str) -> serde_json::Result<Value> {
    let trimmed = raw.trim();
    let unfenced = trimmed
        .strip_prefix("```json")
        .or_else(|| trimmed.strip_prefix("```"))
        .and_then(|rest| rest.strip_suffix("```"))
        .unwrap_or(trimmed);
    serde_json::from_str(unfenced.trim())
}

#[cfg(test)]
mod tests {
    use super::parse_json;

    #[test]
    fn parses_plain_and_fenced_json() {
        assert_eq!(
            parse_json("{\"a\":1}").ok(),
            Some(serde_json::json!({"a": 1}))
        );
        assert_eq!(
            parse_json("```json\n{\"a\":1}\n```").ok(),
            Some(serde_json::json!({"a": 1}))
        );
        assert!(parse_json("not json").is_err());
    }
}
