mod chip8;
mod terminsal_screen;
mod traits;
use std::{thread, time::Duration};

use crate::traits::Screen;
use anyhow::Result;
use ndarray::Array2;
use terminsal_screen::TerminalScreen;

fn main() -> Result<()> {
    let mut memory = Array2::<bool>::from_elem((16, 32), false);

    memory[[5, 5]] = true;
    let mut screen = TerminalScreen::<bool>::new(&memory)?;

    loop {
        screen.draw()?;
        if screen.key_input()? == Some('q') {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
