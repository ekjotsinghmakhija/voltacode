// crates/voltacode-cli/src/main.rs
mod render;

use clap::Parser;
use render::{Spinner, TerminalRenderer};
use std::io::{self, Write};
use voltacode_core::llm::{anthropic::AnthropicClient, openai::OpenAiClient, LlmClient, Message, Role};

#[derive(Parser, Debug)]
#[command(name = "voltacode", about = "High-performance intelligent coding agent")]
struct Cli {
    /// One-shot prompt to execute without entering REPL
    #[arg(short, long)]
    prompt: Option<String>,

    /// Specify LLM provider (anthropic, openai)
    #[arg(long, default_value = "anthropic")]
    provider: String,

    /// Specify model identifier
    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let renderer = TerminalRenderer::new();
    let mut stdout = io::stdout();

    let client: Box<dyn LlmClient> = match cli.provider.as_str() {
        "openai" => Box::new(OpenAiClient::new()),
        _ => Box::new(AnthropicClient::new()),
    };

    if let Some(prompt_text) = cli.prompt {
        execute_prompt(&prompt_text, &*client, &renderer, &mut stdout).await?;
        return Ok(());
    }

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

        execute_prompt(trimmed, &*client, &renderer, &mut stdout).await?;
    }

    Ok(())
}

async fn execute_prompt(
    prompt: &str,
    client: &dyn LlmClient,
    renderer: &TerminalRenderer,
    stdout: &mut std::io::Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut spinner = Spinner::new();
    spinner.tick("Executing...", renderer.color_theme(), stdout)?;

    let messages = vec![Message {
        role: Role::User,
        content: prompt.to_string(),
    }];

    let response = match client.completion(&messages).await {
        Ok(res) => res,
        Err(e) => format!("Execution Error: {}", e),
    };

    spinner.finish("Done", renderer.color_theme(), stdout)?;
    println!("{}\n", renderer.render_markdown(&response));
    Ok(())
}
