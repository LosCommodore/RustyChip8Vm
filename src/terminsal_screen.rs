#![allow(unused)]

use crate::traits::Screen;
use anyhow::Result;

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
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        Ok(Self { terminal })
    }
}

impl Drop for TerminalScreen {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = stdout().execute(crossterm::cursor::Show);
    }
}

impl<'a> Screen for TerminalScreen {
    fn draw(&mut self, mem: &Array2<bool>) -> Result<()> {
        const WHITE: Color = Color::Rgb(255, 255, 255);
        const BLACK: Color = Color::Rgb(0, 0, 0);

        self.terminal.draw(|f| {
            let (height, width) = mem.dim();
            let area = Rect::new(0, 0, width as u16, height as u16);
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
    fn key_input(&mut self) -> Result<Option<char>> {
        if event::poll(std::time::Duration::from_millis(0))? {
            if let event::Event::Key(key) = event::read()? {
                if let KeyCode::Char(c) = key.code {
                    return Ok(Some(c));
                }
            }
        }
        Ok(None)
    }
}
