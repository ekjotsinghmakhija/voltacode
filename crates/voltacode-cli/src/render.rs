// crates/voltacode-cli/src/render.rs
use crossterm::{
    cursor::{MoveToColumn, RestorePosition, SavePosition},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{Clear, ClearType},
};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::io::{self, Write};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

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

    pub fn tick(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
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

    pub fn finish(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
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
    ps: SyntaxSet,
    ts: ThemeSet,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            color_theme: ColorTheme::default(),
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn color_theme(&self) -> &ColorTheme {
        &self.color_theme
    }

    pub fn render_markdown(&self, markdown: &str) -> String {
        let mut output = String::new();
        let parser = Parser::new_ext(markdown, Options::all());
        let mut in_code_block = false;
        let mut current_language = String::new();
        let mut code_buffer = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    current_language = lang.into_string();
                    code_buffer.clear();
                    let lang_label = if current_language.is_empty() {
                        "code"
                    } else {
                        &current_language
                    };
                    output.push_str(&format!("\n╭─ {} ─╮\n", lang_label));
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    let lang = if current_language.is_empty() {
                        "txt"
                    } else {
                        &current_language
                    };
                    let syntax = self
                        .ps
                        .find_syntax_by_token(lang)
                        .unwrap_or_else(|| self.ps.find_syntax_plain_text());

                    let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);
                    for line in LinesWithEndings::from(&code_buffer) {
                        if let Ok(ranges) = h.highlight_line(line, &self.ps) {
                            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                            output.push_str(&format!("│ {}", escaped));
                        } else {
                            output.push_str(&format!("│ {}", line));
                        }
                    }
                    if !code_buffer.ends_with('\n') {
                        output.push('\n');
                    }
                    let width = std::cmp::max(4, current_language.len()) + 4;
                    output.push_str(&format!("╰{}╯\n", "─".repeat(width)));
                }
                Event::Start(Tag::Heading { .. }) => {
                    output.push_str("\n\x1b[1m\x1b[38;5;14m"); // Bold Cyan
                }
                Event::End(TagEnd::Heading(_)) => {
                    output.push_str("\x1b[0m\n");
                }
                Event::Start(Tag::Item) => {
                    output.push_str(" • ");
                }
                Event::Text(text) => {
                    if in_code_block {
                        code_buffer.push_str(&text);
                    } else {
                        output.push_str(&text);
                    }
                }
                Event::Code(code) => {
                    output.push_str(&format!("\x1b[38;5;214m`{}`\x1b[0m", code));
                    // Orange
                }
                Event::SoftBreak | Event::HardBreak => {
                    output.push('\n');
                }
                _ => {}
            }
        }
        output
    }
}
