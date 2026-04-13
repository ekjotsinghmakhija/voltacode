mod render;

use render::{Spinner, TerminalRenderer};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let renderer = TerminalRenderer::new();
    let mut stdout = io::stdout();

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
        for _ in 0..10 {
            spinner.tick("Thinking...", renderer.color_theme(), &mut stdout)?;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        // Bridge: voltacode_core::llm execution context
        let response = format!("Processed: {}", trimmed);

        spinner.finish("Done", renderer.color_theme(), &mut stdout)?;
        println!("{}\n", renderer.render_markdown(&response));
    }

    Ok(())
}
