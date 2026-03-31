mod chip8;
mod terminsal_screen;
mod traits;
use anyhow::Result;
use chip8::Chip8;
use std::fs;
use terminsal_screen::TerminalScreen;

fn main() -> Result<()> {
    let screen = TerminalScreen::new()?;

    let file = "chip_test/chip8-test-rom/test_opcode.ch8";
    let program = fs::read(file)?;
    let mut chip8 = Chip8::new(&program, screen);
    chip8.run()?;
    Ok(())
}
