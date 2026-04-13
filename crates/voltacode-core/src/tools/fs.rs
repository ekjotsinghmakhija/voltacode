// crates/voltacode-core/src/tools/fs.rs
use super::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::fs;

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &'static str {
        "read_file"
    }
    fn description(&self) -> &'static str {
        "Reads the exact contents of a file at the given path."
    }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Absolute or relative path to the file" }
            },
            "required": ["path"]
        })
    }
    async fn execute(&self, args: Value) -> Result<String, String> {
        let path = args["path"].as_str().ok_or("Missing 'path' argument")?;
        fs::read_to_string(path).await.map_err(|e| e.to_string())
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &'static str {
        "write_file"
    }
    fn description(&self) -> &'static str {
        "Writes the provided string content to a file, overwriting existing content."
    }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Target file path" },
                "content": { "type": "string", "description": "Complete file content to write" }
            },
            "required": ["path", "content"]
        })
    }
    async fn execute(&self, args: Value) -> Result<String, String> {
        let path = args["path"].as_str().ok_or("Missing 'path' argument")?;
        let content = args["content"]
            .as_str()
            .ok_or("Missing 'content' argument")?;
        fs::write(path, content).await.map_err(|e| e.to_string())?;
        Ok(format!("Successfully wrote to {}", path))
    }
}
