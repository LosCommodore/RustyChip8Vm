use anyhow::{Context, Result};
use rusty_chip8_vm::chip8::Chip8;
use rusty_chip8_vm::terminal_screen::TerminalScreen;
use std::fs;

fn main() -> Result<()> {
    let screen = TerminalScreen::new()?;

    //let file = "chip_test/chip8-test-rom/test_opcode.ch8";
    //let file = "chip_test/chip8-test-rom-2/chip8-test-rom.ch8";

    let file = "games/INVADERS";
    let program = fs::read(file).with_context(|| format!("Could not read file: {}", file))?;
    let mut chip8 = Chip8::new(&program, screen);
    chip8.run()?;
    Ok(())
}
