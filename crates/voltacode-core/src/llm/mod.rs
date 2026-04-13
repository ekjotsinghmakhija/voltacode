// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
// ============================================================================

pub mod anthropic;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Universal trait for LLM Providers (Anthropic, OpenAI, Local)
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate_response(&self, context: &[Message]) -> Result<String, LlmError>;
    async fn stream_response(&self, context: &[Message]) -> Result<(), LlmError>;
}
