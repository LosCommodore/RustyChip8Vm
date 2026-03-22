const MEM_SIZE: usize = 1024 * 4;

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
    pub general_registers: [u8; 15],
    pub flag_register: u8,
    pub pc: u16,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

#[allow(unused)]
struct Chip8 {
    reg: Registers,
    ram: [u8; 1024 * 4], // 4kb ram, first 512bytes used by VM
}

impl Chip8 {
    #[allow(unused)]
    pub fn new(program: &[u8]) -> Self {
        // Initialize registers
        let mut reg = Registers::default();
        reg.pc = 0x200;

        // Initilize RAM
        let mut ram = [0; MEM_SIZE];
        ram[..FONT_SET.len()].copy_from_slice(&FONT_SET);
        ram[512..512 + program.len()].copy_from_slice(program);
        Self { reg, ram }
    }

    #[allow(unused)]
    pub fn excecute_cmd(&mut self, nibbles: [u8; 4]) {
        match nibbles {
            [0, 0, 0xE, 0] => {
                todo!("Clear scrren")
            }

            [0, 0, 0xE, 0xE] => {
                todo!("Return subroutine")
            }

            [0, a, b, c] => {
                let jump = (a as u16).pow(8) + (b as u16).pow(8) + c as u16;
                self.reg.pc = jump;
                todo!(
                    "
                    Reset timers and registers, 
                    Reset clear screen, 
                    big or little endian ?"
                );
            }

            [1, _a, _b, _c] => {
                todo!("Jump to adress, no resets")
            }

            [2, _a, _b, _c] => {
                todo!("Subroutine at")
            }

            _ => unimplemented!(),
        }
    }
}
