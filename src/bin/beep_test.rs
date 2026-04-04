use std::{thread, time::Duration};

use anyhow::Result;
use rusty_chip8_vm::audio;

fn main() -> Result<()> {
    let beeper = audio::Beeper::new(200f32);
    beeper.player.play();
    thread::sleep(Duration::from_secs(1));
    beeper.player.pause();
    thread::sleep(Duration::from_secs(1));
    beeper.player.play();
    thread::sleep(Duration::from_secs(1));
    Ok(())
}
