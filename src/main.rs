use crossterm::{
    ExecutableCommand,
    event::{self, KeyCode},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, *},
};
use std::io::{Result, stdout};

fn main() -> Result<()> {
    // Ersetze Color::White durch:
    let white = Color::Rgb(255, 255, 255);

    // Ersetze Color::Black durch:
    let black = Color::Rgb(0, 0, 0);

    // Terminal Setup
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // 32x16 Matrix (0 = Schwarz, 1 = Weiß)
    let mut matrix = [[0u8; 32]; 16];

    // Demo-Bild: Ein "X" zeichnen
    for i in 0..16 {
        matrix[i][i * 2] = 1; // Diagonale 1
        matrix[i][31 - (i * 2)] = 1; // Diagonale 2
    }

    loop {
        // 1. Zentriertes Rechteck definieren (32 Spalten * 2 Zeichen breit, 16 Zeilen hoch)
        //let area = Rect::new(0, 0, 64, 16);
        // Nutze f.size() und Layout, falls es in die Mitte soll.
        terminal.draw(|f| {
            // 1. Definiere den Bereich: 32 Pixel * 2 (Breite) = 64 Spalten, 16 Zeilen hoch
            let area = Rect::new(0, 0, 64, 16);

            // 2. Erstelle den Puffer (die Leinwand)
            let mut lines = Vec::new();
            for y in 0..16 {
                let mut spans = Vec::new();
                for x in 0..32 {
                    // Wähle Farbe basierend auf Matrix-Wert
                    let color = if ((x + y) % 2) == 1 { black } else { white };

                    // "  " (zwei Leerzeichen) füllen die Zelle komplett aus
                    spans.push(Span::styled("  ", Style::default().bg(color)));
                }
                lines.push(Line::from(spans));
            }

            // 3. Darstellen (kein Canvas nötig)
            let display = Paragraph::new(lines);
            f.render_widget(display, area);
        })?;

        /*
                      terminal.draw(|f| {

                   let canvas = Canvas::default()
                       .block(Block::default().borders(Borders::ALL).title("32x16 Pixel"))
                       .x_bounds([0.0, 32.0])
                       .y_bounds([0.0, 16.0])
                       .paint(|ctx| {
                           for y in 0..16 {
                               for x in 0..32 {
                                   if x % 2 == 1 {
                                       ctx.print(x as f64, y as f64, "██");
                                   }
                                   /*
                                   if matrix[y][x] == 1 {
                                       // Wir drucken ZWEI Leerzeichen für ein quadratisches Aussehen
                                       ctx.print(x as f64, (15 - y) as f64, "██");
                                   }
                                   */
                               }
                           }
                       });

                   // Render das Widget nur in dem definierten kleinen Bereich
                   f.render_widget(canvas, area);
               })?;
        */
        // Beenden mit 'q'
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
