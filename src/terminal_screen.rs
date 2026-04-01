#![allow(unused)]

use crate::traits::Screen;
use anyhow::Result;
use crossterm::execute;

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ndarray::Array2;
use ratatui::{prelude::*, widgets::*};
use std::io::{Stdout, stdout};
pub struct TerminalScreen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> TerminalScreen {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();

        // 2. Enter Alternate Screen AND push enhancement flags on the SAME handle
        // Note: We use REPORT_EVENT_TYPES to get Press/Repeat/Release info
        execute!(
            stdout,
            EnterAlternateScreen,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }
}

impl Drop for TerminalScreen {
    fn drop(&mut self) {
        // Use a single stdout handle for all cleanup commands
        let mut terminal_out = std::io::stdout();

        // 1. Reset all terminal states in reverse order of initialization
        let _ = execute!(
            terminal_out,
            crossterm::cursor::Show,     // Ensure cursor is visible
            PopKeyboardEnhancementFlags, // Disable the Kitty Protocol
            LeaveAlternateScreen         // Return to the normal shell screen
        );

        // 2. Disable raw mode last (restores normal line buffering)
        let _ = disable_raw_mode();
    }
}

impl<'a> Screen for TerminalScreen {
    fn draw(&mut self, mem: &Array2<bool>) -> Result<()> {
        const WHITE: Color = Color::Rgb(255, 255, 255);
        const BLACK: Color = Color::Rgb(0, 0, 0);

        self.terminal.draw(|f| {
            let (height, width) = mem.dim();
            let area = Rect::new(0, 0, (width as u16) * 2, height as u16); // *2 due to two spaces of an rectangle
            let mut lines = Vec::new();
            for y in 0..height {
                let mut spans = Vec::new();
                for x in 0..width {
                    let color = if mem[[y, x]] { BLACK } else { WHITE };
                    spans.push(Span::styled("  ", Style::default().bg(color)));
                }
                lines.push(Line::from(spans));
            }

            let display = Paragraph::new(lines);
            f.render_widget(display, area);
        })?;

        Ok(())
    }
    fn key_input(&mut self) -> Result<Option<(char, bool)>> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let event::Event::Key(key_event) = event::read()? {
                match key_event.kind {
                    event::KeyEventKind::Press => {
                        if let KeyCode::Char(c) = key_event.code {
                            return Ok(Some((c, true)));
                        }
                    }
                    event::KeyEventKind::Repeat => (),
                    event::KeyEventKind::Release => {
                        if let KeyCode::Char(c) = key_event.code {
                            return Ok(Some((c, false)));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}
