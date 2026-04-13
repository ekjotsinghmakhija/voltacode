// orchestrator/src/tui_monitor.rs
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};
use tokio::sync::mpsc;
use voltacode_core::llm::{anthropic::AnthropicClient, LlmClient, Message, Role};

struct AppState {
    input: String,
    messages: Vec<String>,
}

pub async fn init_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState {
        input: String::new(),
        messages: vec!["[System]: Clean-room orchestration bridge active.".to_string()],
    };

    let res = run_event_loop(&mut terminal, &mut app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

async fn run_event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    loop {
        if let Ok(response) = rx.try_recv() {
            app.messages.push(format!("⚡ {}", response));
        }

        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(75),
                    Constraint::Percentage(20),
                    Constraint::Min(3),
                ])
                .split(size);

            let chat_content = app.messages.join("\n\n");
            let chat_text = Paragraph::new(chat_content).block(
                Block::default()
                    .title(" ⚡ Voltacode Flight Log ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan))
            );
            f.render_widget(chat_text, chunks[0]);

            let input_text = Paragraph::new(format!(">_ {}", app.input)).block(
                Block::default().title(" Command Override ").borders(Borders::ALL)
            );
            f.render_widget(input_text, chunks[1]);

            let hud_text = Paragraph::new(" MODE: Active | PROVIDER: Anthropic | TOKENS: Tracking... ")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().bg(Color::DarkGray).fg(Color::White));
            f.render_widget(hud_text, chunks[2]);
        })?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char(c) => app.input.push(c),
                    KeyCode::Backspace => { app.input.pop(); },
                    KeyCode::Enter => {
                        let prompt = app.input.clone();
                        if prompt.trim().is_empty() { continue; }

                        app.input.clear();
                        app.messages.push(format!("You: {}", prompt));

                        let tx_clone = tx.clone();
                        tokio::spawn(async move {
                            let client = AnthropicClient::new();
                            let msg = vec![Message { role: Role::User, content: prompt }];

                            // Consumes non-Send `e` before the tx_clone await point
                            let result_msg = match client.completion(&msg).await {
                                Ok(res) => res,
                                Err(e) => format!("Error: {}", e),
                            };

                            tx_clone.send(result_msg).await.ok();
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}
