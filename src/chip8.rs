const MEM_SIZE: usize = 1024 * 4;
const SCREEN_WIDTH: usize = 32;
const SCREEN_HEIGHT: usize = 16;
use anyhow::Ok;
use anyhow::Result;
use anyhow::bail;
use ndarray::Array2;
use rand::RngExt;
use std::thread;
use std::time::Duration;

use crate::traits::Screen;

const NR_REGISTERS: usize = 16;

// The font set, hardcoded
const FONT_SET: [u8; 5 * 16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Registers and pseudo registers
#[allow(unused)]
#[derive(Default)]
pub struct Registers {
    pub general_registers: [u8; NR_REGISTERS],
    pub pc: u16,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

#[allow(unused)]
pub struct Chip8<S: Screen> {
    reg: Registers,
    ram: [u8; 1024 * 4], // 4kb ram, first 512bytes used by VM
    screen_mem: Array2<bool>,
    screen: S,
    stack: Vec<u16>,
    update_screen: bool,
}

fn empty_screen() -> Array2<bool> {
    Array2::<bool>::from_elem((SCREEN_HEIGHT, SCREEN_WIDTH), false)
}

fn nibbles_to_u16(a: u8, b: u8, c: u8) -> u16 {
    (a as u16) << 8 + (b as u16) << 4 + (c as u16)
}

impl<S: Screen> Chip8<S> {
    #[allow(unused)]
    pub fn new(program: &[u8], screen: S) -> Self {
        // Initialize registers
        let mut reg = Registers::default();
        reg.pc = 0x200;

        // Initilize RAM
        let mut ram = [0; MEM_SIZE];
        ram[..FONT_SET.len()].copy_from_slice(&FONT_SET);
        ram[512..512 + program.len()].copy_from_slice(program);

        Self {
            reg,
            ram,
            screen,
            screen_mem: empty_screen(),
            stack: Vec::new(),
            update_screen: false,
        }
    }

    fn draw_sprite(&mut self, x: u8, y: u8, height: u8) {
        let x = x as usize;
        let y = y as usize;
        let height = height as usize;
        let mut has_flipped = false;

        for row in 0..height {
            let abs_y = y + row;
            let mem = self.ram[self.reg.i as usize + row];
            for col in 0..8usize {
                let abs_x = x + col;
                let old_bit = self.screen_mem[[abs_y, abs_x]];
                let new_bit = (mem >> (7 - col) & 1) == 1;
                if old_bit != new_bit {
                    has_flipped = true;
                }
                self.screen_mem[[abs_y, abs_x]] = old_bit ^ new_bit;
            }
        }
        self.reg.general_registers[0xF] = has_flipped as u8;
    }
    pub fn step(&mut self) -> Result<()> {
        macro_rules! R {
            ($idx:expr) => {
                self.reg.general_registers[$idx as usize]
            };
        }

        let b0: u8 = self.ram[self.reg.pc as usize];
        let b1 = self.ram[(self.reg.pc + 1) as usize];

        // nibbles
        let n0 = (b0 & 0xF0) >> 4;
        let n1 = b0 & 0x0F;
        let n2 = (b1 & 0xF0) >> 4;
        let n3 = b1 & 0x0F;

        let mut has_jumped = false;
        match [n0, n1, n2, n3] {
            [0, 0, 0xE, 0] => {
                self.screen_mem = empty_screen();
                self.update_screen = true
            }
            [0, 0, 0xE, 0xE] => {
                // Return from a subroutine
                let Some(new_pc) = self.stack.pop() else {
                    bail!("Stack empty!");
                };
                self.reg.pc = new_pc;
                has_jumped = true;
            }

            [0, a, b, c] => {
                /*
                - Jump
                - Reset timers and registers,
                - Reset clear screen,
                */
                let jump = nibbles_to_u16(a, b, c);
                self.reg.pc = jump;
                has_jumped = true;

                self.reg.delay_timer = 0;
                self.reg.sound_timer = 0;
                self.reg.i = 0;
                self.reg.general_registers = [0; NR_REGISTERS];
            }

            [1, a, b, c] => {
                // Jump
                let jump = nibbles_to_u16(a, b, c);
                self.reg.pc = jump;
                has_jumped = true;
            }

            [2, a, b, c] => {
                // Call subroutine
                self.stack.push(self.reg.pc + 2);
                let jump = nibbles_to_u16(a, b, c);
                self.reg.pc = jump;
                has_jumped = true;
            }

            [3, x, _, _] => {
                // skip next instruction if v[x] equals ab
                if R![x] == b1 {
                    self.reg.pc += 4;
                    has_jumped = true;
                }
            }

            [4, x, _, _] => {
                // skip next instruction if v[x] not equals b1
                if R![x] != b1 {
                    self.reg.pc += 4;
                    has_jumped = true;
                }
            }

            [5, x, y, _] => {
                // skip next instruction if  v[x] = v[y]
                if R![x] == R![y] {
                    self.reg.pc += 4;
                    has_jumped = true;
                }
            }

            [6, x, _, _] => R![x] = b1,
            [7, x, _, _] => R![x] = R![x].wrapping_add(b1),
            [8, x, y, 0] => R![x] = R![y],
            [8, x, y, 1] => R![x] |= R![y],
            [8, x, y, 2] => R![x] &= R![y],
            [8, x, y, 3] => R![x] ^= R![y],
            [8, x, y, 4] => {
                let (value, overflow) = R![x].overflowing_add(R![y]); // add with overflow and carry flag logic 
                R![x] = value;
                R![0xF] = overflow as u8;
            }

            [8, x, y, 5] => {
                let (value, underflow) = R![x].overflowing_sub(R![y]);
                R![x] = value;
                R![0xF] = if !underflow { 1 } else { 0 };
            }

            [8, x, _, 6] => {
                let lsb = R![x] & 1;
                R![x] >>= 1;
                R![0xF] = lsb;
            }

            [8, x, y, 7] => {
                let (value, underflow) = R![y].overflowing_sub(R![x]);
                R![x] = value;
                R![0xF] = if !underflow { 1 } else { 0 };
            }

            [8, x, _, 0xE] => {
                let msb = (R![x] >> 7) & 1;
                R![x] <<= 1;
                R![0xF] = msb;
            }

            [9, x, y, 0] => {
                if R![x] != R![y] {
                    self.reg.pc += 4;
                    has_jumped = true;
                }
            }

            [0xA, a, b, c] => {
                let value = nibbles_to_u16(a, b, c);
                self.reg.i = value;
            }

            [0xB, a, b, c] => {
                let value = nibbles_to_u16(a, b, c);
                self.reg.pc = value + R![0] as u16;
                has_jumped = true;
            }

            [0xC, x, _, _] => {
                let mut rng = rand::rng();
                let n: u8 = rng.random();
                R![x] = n & b1;
            }

            [0xD, x, y, height] => {
                let p_x = R![x];
                let p_y = R![y];
                self.draw_sprite(p_x, p_y, height);
                self.update_screen = true
            }

            _ => unimplemented!(),
        }

        if !has_jumped {
            self.reg.pc += 2;
        };
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.screen.draw(&self.screen_mem)?;
            self.step()?;
            if self.screen.key_input()? == Some('q') {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}
