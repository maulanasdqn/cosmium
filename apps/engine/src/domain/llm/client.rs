use async_trait::async_trait;

use super::error::LlmResult;
use super::messages::{ChatRequest, ChatResponse};

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat(&self, request: ChatRequest) -> LlmResult<ChatResponse>;
}
