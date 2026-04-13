// crates/voltacode-cli/src/main.rs
mod render;
mod input;

use clap::Parser;
use crossterm::style::Stylize;
use dialoguer::{theme::ColorfulTheme, Select};
use input::{LineEditor, ReadOutcome};
use render::{Spinner, TerminalRenderer};
use std::io;
use voltacode_core::llm::{anthropic::AnthropicClient, ollama::OllamaClient, openai::OpenAiClient, LlmClient, Message, Role};

#[derive(Parser, Debug)]
#[command(name = "voltacode", about = "High-performance intelligent coding agent")]
struct Cli {
    #[arg(short, long)]
    prompt: Option<String>,

    #[arg(long)]
    provider: Option<String>,

    #[arg(short, long)]
    model: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let renderer = TerminalRenderer::new();
    let mut stdout = io::stdout();

    let (client, provider_name): (Box<dyn LlmClient>, String) = if let Some(provider) = cli.provider {
        match provider.as_str() {
            "openai" => (Box::new(OpenAiClient::new()), "OpenAI".to_string()),
            "ollama" => {
                let m = cli.model.unwrap_or_else(|| "deepseek-coder:6.7b".to_string());
                (Box::new(OllamaClient::new(m.clone())), format!("Ollama ({})", m))
            },
            _ => (Box::new(AnthropicClient::new()), "Anthropic".to_string()),
        }
    } else {
        let providers = vec![
            "Anthropic (Online - Claude 3.5 Sonnet)",
            "OpenAI (Online - GPT-4o)",
            "Ollama (Local Models)",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select model to run")
            .default(0)
            .items(&providers)
            .interact()?;

        match selection {
            0 => (Box::new(AnthropicClient::new()), "Anthropic".to_string()),
            1 => (Box::new(OpenAiClient::new()), "OpenAI".to_string()),
            2 => {
                let local_models = vec!["deepseek-coder:6.7b", "llama3.1", "qwen2.5-coder:7b"];
                let model_idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select local model")
                    .default(0)
                    .items(&local_models)
                    .interact()?;
                let m = local_models[model_idx].to_string();
                (Box::new(OllamaClient::new(m.clone())), format!("Ollama ({})", m))
            },
            _ => unreachable!(),
        }
    };

    if let Some(prompt_text) = cli.prompt {
        execute_prompt(&prompt_text, &*client, &renderer, &mut stdout).await?;
        return Ok(());
    }

    println!("{}", "╭──────────────────────────────────────────────────╮".with(crossterm::style::Color::DarkGrey));
    println!("{} {} {}",
        "│".with(crossterm::style::Color::DarkGrey),
        "⚡ VOLTACODE AGENT - CLEAN ROOM ORCHESTRATOR ⚡ ".with(crossterm::style::Color::Cyan),
        "│".with(crossterm::style::Color::DarkGrey)
    );
    println!("{}", "├──────────────────────────────────────────────────┤".with(crossterm::style::Color::DarkGrey));
    println!("{} {:<48} {}",
        "│".with(crossterm::style::Color::DarkGrey),
        format!("Provider: {}", provider_name).with(crossterm::style::Color::Yellow),
        "│".with(crossterm::style::Color::DarkGrey)
    );
    println!("{}", "╰──────────────────────────────────────────────────╯\n".with(crossterm::style::Color::DarkGrey));
    println!("Type /exit to quit.\n");

    let mut editor = LineEditor::new("> ");

    loop {
        match editor.read_line()? {
            ReadOutcome::Submit(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if trimmed == "/exit" || trimmed == "/quit" {
                    break;
                }

                execute_prompt(trimmed, &*client, &renderer, &mut stdout).await?;
            }
            ReadOutcome::Cancel => continue,
            ReadOutcome::Exit => break,
        }
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
    spinner.tick("Agent is thinking...", renderer.color_theme(), stdout)?;

    let messages = vec![Message {
        role: Role::User,
        content: prompt.to_string(),
    }];

    let response = match client.completion(&messages).await {
        Ok(res) => res,
        Err(e) => format!("Execution Error: {}", e),
    };

    spinner.finish("Response generated", renderer.color_theme(), stdout)?;
    println!("{}\n", renderer.render_markdown(&response));
    Ok(())
}
