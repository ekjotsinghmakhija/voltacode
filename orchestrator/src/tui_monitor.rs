// orchestrator/tui_monitor.rs
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

pub async fn init_tui() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_event_loop(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

async fn run_event_loop<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Split screen into Chat (75%), Input (20%), Status/HUD (5%)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(75),
                    Constraint::Percentage(20),
                    Constraint::Min(3),
                ])
                .split(size);

            // Chat View
            let chat_block = Block::default()
                .title(" ⚡ Voltacode Flight Log ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let chat_text = Paragraph::new("Awaiting agent telemetry...\n[System]: Clean-room orchestration bridge active.").block(chat_block);
            f.render_widget(chat_text, chunks[0]);

            // Input View
            let input_block = Block::default()
                .title(" Command Override ")
                .borders(Borders::ALL);
            let input_text = Paragraph::new(">_ ").block(input_block);
            f.render_widget(input_text, chunks[1]);

            // HUD / Status Bar
            let hud_block = Block::default().borders(Borders::ALL);
            let hud_text = Paragraph::new(" MODE: Idle | PROVIDER: Auto | TOKENS: 0 | COST: $0.0000 ")
                .block(hud_block)
                .style(Style::default().bg(Color::DarkGray).fg(Color::White));
            f.render_widget(hud_text, chunks[2]);
        })?;

        // Non-blocking event poll
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Char('q') => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}
