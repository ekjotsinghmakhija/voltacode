// ============================================================================
// Copyright (c) 2026 Ekjot Singh
// Contact: ekjotmakhija@gmail.com
// GitHub: https://github.com/ekjotsinghmakhija
//
// Project: Voltacode
// Description: High-performance intelligent coding agent and terminal UI.
// ============================================================================

use std::process::Stdio;
use tokio::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Subprocess execution failed: {0}")]
    Execution(String),
    #[error("Python bridge returned an error: {0}")]
    ScriptFailed(String),
}

/// Executes a Python script via the activated virtual environment bridge.
pub async fn execute_python_tool(script_path: &str, args: &[&str]) -> Result<String, BridgeError> {
    println!("[BRIDGE] Invoking Python Subprocess: {}", script_path);

    // Assuming the user runs the CLI from the root workspace where .venv exists
    let python_bin = ".venv/bin/python";

    let output = Command::new(python_bin)
        .arg(script_path)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| BridgeError::Execution(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        return Err(BridgeError::ScriptFailed(stderr));
    }

    Ok(stdout)
}
