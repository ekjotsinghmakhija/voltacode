// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
// ============================================================================

use super::{LlmError, LlmProvider, Message};
use reqwest::{Client, header};
use serde_json::json;

pub struct AnthropicClient {
    api_key: String,
    model: String,
    http_client: Client,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "x-api-key",
            header::HeaderValue::from_str(&api_key).expect("Invalid API Key format"),
        );
        headers.insert(
            "anthropic-version",
            header::HeaderValue::from_static("2023-06-01"),
        );
        headers.insert(
            "content-type",
            header::HeaderValue::from_static("application/json"),
        );

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key,
            model: model.to_string(),
            http_client,
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for AnthropicClient {
    async fn generate_response(&self, context: &[Message]) -> Result<String, LlmError> {
        println!("[LLM] Dispatching HTTP request to Anthropic model: {}", self.model);

        let payload = json!({
            "model": self.model,
            "max_tokens": 4096,
            "messages": context
        });

        let response = self.http_client
            .post("https://api.anthropic.com/v1/messages")
            .json(&payload)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let err_text = response.text().await.unwrap_or_default();
            return Err(LlmError::Api(format!("API Error: {}", err_text)));
        }

        // For now, we just acknowledge a successful connection setup.
        // Full JSON parsing of the Anthropic response block comes in the execution phase.
        Ok("[LLM] Network Layer connected successfully.".to_string())
    }

    async fn stream_response(&self, _context: &[Message]) -> Result<(), LlmError> {
        println!("[LLM] Opening Server-Sent Events (SSE) stream...");
        Ok(())
    }
}
