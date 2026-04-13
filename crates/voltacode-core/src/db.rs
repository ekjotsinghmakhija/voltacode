use crate::llm::Message;
use std::path::PathBuf;
use tokio::fs;

pub struct SessionStore {
    file_path: PathBuf,
}

impl SessionStore {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: PathBuf::from(path),
        }
    }

    pub async fn save(&self, messages: &[Message]) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(messages)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&self.file_path, data).await
    }

    pub async fn load(&self) -> Result<Vec<Message>, std::io::Error> {
        if !self.file_path.exists() {
            return Ok(Vec::new());
        }
        let data = fs::read_to_string(&self.file_path).await?;
        let messages = serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(messages)
    }
}
