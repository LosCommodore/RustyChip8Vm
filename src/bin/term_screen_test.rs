use anyhow::Result;
//use rusty_chip8_vm::chip8::Chip8;
use ndarray::Array2;
use rusty_chip8_vm::{terminal_screen::TerminalScreen, traits::Screen};

const SCREEN_WIDTH: usize = 16;
const SCREEN_HEIGHT: usize = 32;

fn main() -> Result<()> {
    let mut mem = Array2::<bool>::from_elem((SCREEN_HEIGHT, SCREEN_WIDTH), false);
    for i in 0..SCREEN_WIDTH {
        if i % 2 == 0 {
            mem[(4, i as usize)] = true;
        }
    }

    let mut screen = TerminalScreen::new()?;
    screen.draw(&mem)?;
    loop {
        if let Some(('q', true)) = screen.key_input()? {
            break;
        };
    }
    Ok(())
}
