mod chip8;
mod terminsal_screen;
mod traits;
use anyhow::Result;
use chip8::Chip8;
use terminsal_screen::TerminalScreen;

fn main() -> Result<()> {
    let screen = TerminalScreen::new()?;
    let program = [0u8; 42];

    let mut chip8 = Chip8::new(&program, screen);
    chip8.run()
}
