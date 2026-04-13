// crates/voltacode-core/src/tools/bash.rs
use super::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use tokio::process::Command;

pub struct ExecuteBashTool;

#[async_trait]
impl Tool for ExecuteBashTool {
    fn name(&self) -> &'static str { "execute_bash" }
    fn description(&self) -> &'static str { "Executes a raw bash command in the workspace and returns stdout/stderr." }
    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": { "type": "string", "description": "Bash command string to execute" }
            },
            "required": ["command"]
        })
    }
    async fn execute(&self, args: Value) -> Result<String, String> {
        let cmd = args["command"].as_str().ok_or("Missing 'command' argument")?;
        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .await
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let mut result = String::new();
        if !stdout.is_empty() { result.push_str(&format!("STDOUT:\n{}\n", stdout)); }
        if !stderr.is_empty() { result.push_str(&format!("STDERR:\n{}\n", stderr)); }

        Ok(if result.is_empty() { "Command executed successfully with no output.".to_string() } else { result })
    }
}
