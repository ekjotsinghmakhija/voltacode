// crates/voltacode-core/src/llm/mod.rs
pub mod anthropic;
pub mod openai;
pub mod ollama;


use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[async_trait]
pub trait LlmClient {
    async fn completion(&self, messages: &[Message]) -> Result<String, Box<dyn std::error::Error>>;
}
