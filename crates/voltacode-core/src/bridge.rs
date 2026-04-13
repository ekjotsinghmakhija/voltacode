use std::path::Path;
use tokio::fs;
use std::process::Stdio;
use tokio::process::Command;

pub struct SystemBridge;

impl SystemBridge {
    pub async fn read_file(path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        fs::read_to_string(path).await
    }

    pub async fn write_file(path: impl AsRef<Path>, content: &str) -> Result<(), std::io::Error> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(path, content).await
    }

    pub async fn execute_bash(cmd: &str) -> Result<(String, String, bool), std::io::Error> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        Ok((
            String::from_utf8_lossy(&output.stdout).to_string(),
            String::from_utf8_lossy(&output.stderr).to_string(),
            output.status.success()
        ))
    }
}
