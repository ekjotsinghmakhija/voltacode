// crates/voltacode-core/src/llm/anthropic.rs
use super::{LlmClient, Message, Role};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;

pub struct AnthropicClient {
    pub client: Client,
    pub api_key: String,
    pub model: String,
}

impl AnthropicClient {
    pub fn new() -> Self {
        let api_key = env::var("ANTHROPIC_API_KEY").unwrap_or_default();
        Self {
            client: Client::new(),
            api_key,
            model: "claude-3-5-sonnet-20241022".to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn completion(&self, messages: &[Message]) -> Result<String, Box<dyn std::error::Error>> {
        let formatted_messages: Vec<_> = messages.iter().filter(|m| !matches!(m.role, Role::System)).map(|m| {
            json!({
                "role": match m.role {
                    Role::User => "user",
                    Role::Assistant => "assistant",
                    _ => "user",
                },
                "content": m.content
            })
        }).collect();

        let system_prompt = messages.iter().find(|m| matches!(m.role, Role::System)).map(|m| m.content.clone()).unwrap_or_default();

        let payload = json!({
            "model": self.model,
            "max_tokens": 4096,
            "system": system_prompt,
            "messages": formatted_messages,
        });

        let resp = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await?;
            return Err(format!("API Error: {}", err_text).into());
        }

        let data: serde_json::Value = resp.json().await?;
        let content = data["content"][0]["text"].as_str().unwrap_or("").to_string();

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_initialization_parity() {
        std::env::set_var("ANTHROPIC_API_KEY", "mock_token_0x1");
        let client = AnthropicClient::new();
        assert_eq!(client.api_key, "mock_token_0x1");
        assert_eq!(client.model, "claude-3-5-sonnet-20241022");
    }
}
