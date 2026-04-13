// crates/voltacode-cli/src/main.rs
mod render;

use render::{Spinner, TerminalRenderer};
use std::io::{self, Write};
use voltacode_core::llm::{anthropic::AnthropicClient, LlmClient, Message, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = TerminalRenderer::new();
    let mut stdout = io::stdout();
    let client = AnthropicClient::new();

    println!("⚡ Voltacode REPL ⚡");
    println!("Type /exit to quit.");

    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "/exit" || trimmed == "/quit" {
            break;
        }

        let mut spinner = Spinner::new();
        spinner.tick("Executing...", renderer.color_theme(), &mut stdout)?;

        let messages = vec![Message {
            role: Role::User,
            content: trimmed.to_string(),
        }];

        let response = match client.completion(&messages).await {
            Ok(res) => res,
            Err(e) => format!("Execution Error: {}", e),
        };

        spinner.finish("Done", renderer.color_theme(), &mut stdout)?;
        println!("{}\n", renderer.render_markdown(&response));
    }

    Ok(())
}
