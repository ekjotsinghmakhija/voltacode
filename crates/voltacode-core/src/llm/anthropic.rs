// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
// ============================================================================

use super::{LlmError, LlmProvider, Message};

pub struct AnthropicClient {
    api_key: String,
    model: String,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: &str) -> Self {
        Self {
            api_key,
            model: model.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for AnthropicClient {
    async fn generate_response(&self, _context: &[Message]) -> Result<String, LlmError> {
        // HTTP client logic will be injected here
        println!("[LLM] Dispatching request to Anthropic model: {}", self.model);
        Ok("Mocked response ready for core wiring.".to_string())
    }

    async fn stream_response(&self, _context: &[Message]) -> Result<(), LlmError> {
        println!("[LLM] Opening Server-Sent Events (SSE) stream...");
        Ok(())
    }
}
