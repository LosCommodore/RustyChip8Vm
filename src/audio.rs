/*
the system library `alsa` required by crate `alsa-sys` was not found.
  The file `alsa.pc` needs to be installed and the PKG_CONFIG_PATH environment variable must contain its parent directory.
  The PKG_CONFIG_PATH environment variable is not set.
This happens because Rodio depends on cpal, which requires the ALSA development headers to talk to the Linux sound system. Even though it's "pure Rust," it still needs to link to the OS audio kernel.
To fix this, install the ALSA development package:
1. The Fix (Debian/Ubuntu/Mint)
Run this in your terminal:
bash
sudo apt update
sudo apt install libasound2-dev
s*/

use rodio::source::{Source, SquareWave};
use rodio::{MixerDeviceSink, Player};

#[allow(unused)]
pub struct Beeper {
    pub player: Player,
    freq: f32,
    device_sink: MixerDeviceSink, // Note that playback through Player will end if the associated DeviceSink is dropped.
}

impl Beeper {
    pub fn new(freq: f32) -> Self {
        let handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&handle.mixer());
        player.pause();

        let source = SquareWave::new(freq).amplify(0.20);
        player.append(source);

        Beeper {
            freq,
            player,
            device_sink: handle,
        }
    }
}
