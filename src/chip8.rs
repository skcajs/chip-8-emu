use std::time::{SystemTime, UNIX_EPOCH};
use rand::{rngs::StdRng, SeedableRng};


pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    gfx: [u8; 64*32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: usize,
    key: [u8; 16]
}

const CHIP8_FONTSET: [u8; 80] =
[ 
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
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl Chip8 {
    pub fn new() -> Self {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Use the current time as a seed
        let seed = current_time as u64;

        // Initialize the random number generator with the seed
        let mut rng = StdRng::seed_from_u64(seed);

        let mut memory = [0; 4096];

        for (i, elem) in CHIP8_FONTSET.iter().enumerate() {
            memory[i] = *elem;
        }
        
        Chip8 {
            opcode: 0x200,
            memory,
            v: [0; 16],
            i: 0,
            pc: 0x2000,
            gfx: [0; 64*32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            key: [0; 16]
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = u16::from(self.memory[self.pc as usize]) << 8 | u16::from(self.memory[self.pc as usize + 1]);
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    0x0000 => { // Clear screen
                        for element in self.gfx.iter_mut() {
                            *element = 0;
                        };
                    }
                    0x000E => {

                    }
                    _ => {
                        
                    }
                }
                self.increment_pc();
            }
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }
            0x2000 => {
                self.stack[self.stack_pointer] = self.pc;
                self.stack_pointer += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x3000 => {
                if self.v[(self.opcode & 0x0F00 >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x4000 => {
                if self.v[(self.opcode & 0x0F00 >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x5000 => {
                if self.v[(self.opcode & 0x0F00 >> 8) as usize] == self.v[(self.opcode & 0x00F0 >> 4) as usize] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0x6000 => {
                self.v[(self.opcode & 0x0F00 >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.increment_pc();

            }
            0x7000 => {
                let x = (self.opcode & 0x0F00 >> 8) as usize;
                self.v[x] += (self.opcode & 0x00FF) as u8;
                self.increment_pc();

            }
            0x8000 => {
                let x = (self.opcode & 0x0F00 >> 8) as usize;
                let y = (self.opcode & 0x00F0 >> 4) as usize;
                match self.opcode & 0x000F {
                    0x0000 => {
                        self.v[x] = self.v[y];
                    }
                    0x0001 => {
                        self.v[x] |= self.v[y];
                    }
                    0x0002 => {
                        self.v[x] &= self.v[y];
                    }
                    0x0003 => {
                        self.v[x] ^= self.v[y];
                    }

                    0x0004 => {
                        self.v[0xF] = if self.v[y] > 0xFF - self.v[x] { 1 } else { 0 };
                        self.v[x] += self.v[y];

                    }
                    0x0005 => {
                        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
                        self.v[x] -= self.v[y];

                    }
                    0x0006 => {
                        self.v[0xF] = self.v[x] & 1;
                        self.v[x] >>= 1;

                    }
                    0x0007 => {
                        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
                        self.v[x] = self.v[y] - self.v[x];

                    }
                    0x000E => {
                        self.v[0xF] = if self.v[x] & 0x80 != 0 { 1 } else {0};
                        self.v[x] <<= 1;

                    }
                    _ => {

                    }
                }
                self.increment_pc();

            }
            0x9000 => {
                if self.v[(self.opcode & 0x0F00 >> 8) as usize] != self.v[(self.opcode & 0x00F0 >> 4) as usize] {
                    self.increment_pc();
                }
                self.increment_pc();
            }
            0xA000 => {
                self.i = self.opcode & 0x0FFF;
                self.increment_pc();
            }
            0xB000 => {
                self.pc = self.v[0] as u16 + self.opcode & 0x0FFF;
            }
            _ => {

            }
        }
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}