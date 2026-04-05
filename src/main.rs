use anyhow::{Context, Result};
use rusty_chip8_vm::chip8::Chip8;
use rusty_chip8_vm::terminal_screen::TerminalScreen;
use std::{env, fs, process};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Missing parameter: progam file");
        process::exit(1);
    }

    let file = &args[1];
    let screen = TerminalScreen::new()?;
    //let file = "games/INVADERS";
    let program = fs::read(file).with_context(|| format!("Could not read file: {}", file))?;
    let mut chip8 = Chip8::new(&program, screen);
    chip8.run()?;
    Ok(())
}
