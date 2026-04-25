use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::domain::llm::{
    ChatMessage, ChatRequest, ChatResponse, LlmClient, LlmError, LlmResult, Role,
    messages::Usage,
};

pub struct OpenRouterClient {
    http: Client,
    base_url: String,
    api_key: String,
    referer: Option<String>,
    title: Option<String>,
}

pub struct OpenRouterConfig {
    pub api_key: String,
    pub base_url: String,
    pub referer: Option<String>,
    pub title: Option<String>,
}

impl OpenRouterClient {
    pub fn new(cfg: OpenRouterConfig) -> Self {
        Self {
            http: Client::builder()
                .user_agent("cosmium/0.1")
                .build()
                .expect("build reqwest client"),
            base_url: cfg.base_url,
            api_key: cfg.api_key,
            referer: cfg.referer,
            title: cfg.title,
        }
    }
}

#[async_trait]
impl LlmClient for OpenRouterClient {
    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse> {
        let body = WireRequest {
            model: &request.model,
            messages: request.messages.iter().map(WireMessage::from).collect(),
            temperature: request.temperature,
            response_format: request.json_mode.then_some(WireFormat { kind: "json_object" }),
        };

        let mut req = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body);
        if let Some(r) = &self.referer {
            req = req.header("HTTP-Referer", r);
        }
        if let Some(t) = &self.title {
            req = req.header("X-Title", t);
        }

        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(LlmError::Provider {
                status: status.as_u16(),
                body,
            });
        }

        let parsed: WireResponse = resp.json().await?;
        let choice = parsed.choices.into_iter().next().ok_or(LlmError::EmptyResponse)?;
        Ok(ChatResponse {
            content: choice.message.content,
            model: parsed.model,
            usage: parsed.usage.map(|u| Usage {
                prompt_tokens: u.prompt_tokens,
                completion_tokens: u.completion_tokens,
                total_tokens: u.total_tokens,
            }),
        })
    }
}

#[derive(Serialize)]
struct WireRequest<'a> {
    model: &'a str,
    messages: Vec<WireMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<WireFormat>,
}

#[derive(Serialize)]
struct WireFormat {
    #[serde(rename = "type")]
    kind: &'static str,
}

#[derive(Serialize)]
struct WireMessage {
    role: &'static str,
    content: String,
}

impl From<&ChatMessage> for WireMessage {
    fn from(m: &ChatMessage) -> Self {
        Self {
            role: match m.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
            },
            content: m.content.clone(),
        }
    }
}

#[derive(Deserialize)]
struct WireResponse {
    model: String,
    choices: Vec<WireChoice>,
    usage: Option<WireUsage>,
}

#[derive(Deserialize)]
struct WireChoice {
    message: WireResponseMessage,
}

#[derive(Deserialize)]
struct WireResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct WireUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
