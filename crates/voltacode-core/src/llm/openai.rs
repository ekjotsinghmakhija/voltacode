// crates/voltacode-core/src/llm/openai.rs
use super::{LlmClient, Message, Role};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::env;

pub struct OpenAiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
        Self {
            client: Client::new(),
            api_key,
            model: "gpt-4o".to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for OpenAiClient {
    async fn completion(&self, messages: &[Message]) -> Result<String, Box<dyn std::error::Error>> {
        let formatted_messages: Vec<_> = messages.iter().map(|m| {
            json!({
                "role": match m.role {
                    Role::System => "system",
                    Role::User => "user",
                    Role::Assistant => "assistant",
                },
                "content": m.content
            })
        }).collect();

        let payload = json!({
            "model": self.model,
            "messages": formatted_messages,
        });

        let resp = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .header("content-type", "application/json")
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await?;
            return Err(format!("OpenAI API Error: {}", err_text).into());
        }

        let data: serde_json::Value = resp.json().await?;
        let content = data["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();

        Ok(content)
    }
}
