// crates/voltacode-core/src/bridge.rs
use crate::llm::{LlmClient, Message, Role};
use crate::tools::ToolRegistry;
use tokio::sync::mpsc::Sender;

pub struct AgentBridge<'a> {
    client: &'a dyn LlmClient,
    registry: ToolRegistry,
}

impl<'a> AgentBridge<'a> {
    pub fn new(client: &'a dyn LlmClient, registry: ToolRegistry) -> Self {
        Self { client, registry }
    }

    pub async fn execute(
        &self,
        prompt: &str,
        tx: Sender<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut messages = vec![Message {
            role: Role::User,
            content: prompt.to_string(),
        }];

        let schemas = self.registry.get_schemas();
        tx.send(format!(
            "[System] Agent initialized with {} tools.",
            schemas.len()
        ))
        .await
        .ok();

        // LLM multi-turn execution loop stub
        let response = self.client.completion(&messages).await?;

        // TODO: Phase 4.1 - Parse response for tool execution schemas
        // self.registry.execute(name, args).await;
        // messages.push(ToolResult);

        tx.send(response).await.ok();

        Ok(())
    }
}
