pub mod anthropic;

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
