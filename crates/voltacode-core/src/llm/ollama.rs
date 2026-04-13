// crates/voltacode-core/src/llm/ollama.rs
use super::{LlmClient, Message, Role};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

pub struct OllamaClient {
    pub client: Client,
    pub model: String,
    pub base_url: String,
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        Self {
            client: Client::new(),
            model,
            base_url: "http://localhost:11434".to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for OllamaClient {
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
            "stream": false
        });

        let resp = self.client.post(&format!("{}/api/chat", self.base_url))
            .json(&payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err_text = resp.text().await?;
            return Err(format!("Ollama API Error: {}", err_text).into());
        }

        let data: serde_json::Value = resp.json().await?;
        let content = data["message"]["content"].as_str().unwrap_or("").to_string();

        Ok(content)
    }
}
