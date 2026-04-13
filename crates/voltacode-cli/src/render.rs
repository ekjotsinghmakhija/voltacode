use crossterm::{
    cursor::{MoveToColumn, RestorePosition, SavePosition},
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
    execute, queue,
};
use std::io::{self, Write};
use pulldown_cmark::{Event, Options, Parser};

pub struct ColorTheme {
    pub spinner_active: Color,
    pub spinner_done: Color,
    pub spinner_failed: Color,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            spinner_active: Color::Blue,
            spinner_done: Color::Green,
            spinner_failed: Color::Red,
        }
    }
}

pub struct Spinner {
    frame_index: usize,
}

impl Spinner {
    const FRAMES: [&'static str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    pub fn new() -> Self {
        Self { frame_index: 0 }
    }

    pub fn tick(&mut self, label: &str, theme: &ColorTheme, out: &mut impl Write) -> io::Result<()> {
        let frame = Self::FRAMES[self.frame_index % Self::FRAMES.len()];
        self.frame_index += 1;
        queue!(
            out,
            SavePosition,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_active),
            Print(format!("{frame} {label}")),
            ResetColor,
            RestorePosition
        )?;
        out.flush()
    }

    pub fn finish(&mut self, label: &str, theme: &ColorTheme, out: &mut impl Write) -> io::Result<()> {
        self.frame_index = 0;
        execute!(
            out,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_done),
            Print(format!("✔ {label}\n")),
            ResetColor
        )?;
        out.flush()
    }
}

pub struct TerminalRenderer {
    color_theme: ColorTheme,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            color_theme: ColorTheme::default(),
        }
    }

    pub fn color_theme(&self) -> &ColorTheme {
        &self.color_theme
    }

    pub fn render_markdown(&self, markdown: &str) -> String {
        let mut output = String::new();
        let parser = Parser::new_ext(markdown, Options::all());

        for event in parser {
            match event {
                Event::Text(text) => output.push_str(&text),
                Event::SoftBreak | Event::HardBreak => output.push('\n'),
                Event::Code(code) => output.push_str(&format!("{}", format!("`{code}`").with(Color::Green))),
                _ => {} // Target: minimal plain-text representation fallback
            }
        }
        output
    }
}
