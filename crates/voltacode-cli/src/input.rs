// crates/voltacode-cli/src/input.rs
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;
use std::io::{self, IsTerminal};

pub enum ReadOutcome {
    Submit(String),
    Cancel,
    Exit,
}

pub struct LineEditor {
    editor: Editor<(), DefaultHistory>,
    prompt: String,
}

impl LineEditor {
    pub fn new(prompt: &str) -> Self {
        let editor = Editor::<(), DefaultHistory>::new().expect("Failed to initialize line editor");
        Self {
            editor,
            prompt: prompt.to_string(),
        }
    }

    pub fn read_line(&mut self) -> io::Result<ReadOutcome> {
        if !io::stdin().is_terminal() {
            let mut buffer = String::new();
            let bytes = io::stdin().read_line(&mut buffer)?;
            if bytes == 0 {
                return Ok(ReadOutcome::Exit);
            }
            return Ok(ReadOutcome::Submit(buffer.trim_end().to_string()));
        }

        match self.editor.readline(&self.prompt) {
            Ok(line) => {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    let _ = self.editor.add_history_entry(trimmed);
                }
                Ok(ReadOutcome::Submit(line))
            }
            Err(ReadlineError::Interrupted) => Ok(ReadOutcome::Cancel),
            Err(ReadlineError::Eof) => Ok(ReadOutcome::Exit),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }
}
