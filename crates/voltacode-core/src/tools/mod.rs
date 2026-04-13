// crates/voltacode-core/src/tools/mod.rs
pub mod bash;
pub mod fs;

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn schema(&self) -> Value;
    async fn execute(&self, args: Value) -> Result<String, String>;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub async fn execute(&self, name: &str, args: Value) -> Result<String, String> {
        if let Some(tool) = self.tools.get(name) {
            tool.execute(args).await
        } else {
            Err(format!("Tool '{}' not found", name))
        }
    }

    pub fn get_schemas(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|t| {
                serde_json::json!({
                    "name": t.name(),
                    "description": t.description(),
                    "input_schema": t.schema()
                })
            })
            .collect()
    }
}
