use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use rand::{rng, Rng};
use std::fs::File;
use std::io::Read;

const START_ADDRESS: u16 = 0x200;

const FONT_SET_START_ADDRESS: u16 = 0x050;
const FONT_SET: [u8; 80] = [
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

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video_frame_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    opcode: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0; 4096];

        for i in 0..FONT_SET.len() {
            memory[FONT_SET_START_ADDRESS as usize + i] = FONT_SET[i];
        }

        Self {
            registers: [0x0; 16],
            memory,
            index: 0x0,
            pc: START_ADDRESS,
            stack: [0x0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video_frame_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            opcode: 0x0,
        }
    }

    pub fn load_rom(&mut self, filename: &str) {
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut file = File::open(filename).unwrap();
            file.read_to_end(&mut buf).unwrap();
        }

        for i in 0..buf.len() {
            self.memory[START_ADDRESS as usize + i] = buf[i];
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn execute_cycle(&mut self) {
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8)
            | self.memory[(self.pc + 1) as usize] as u16;

        self.pc += 2;

        self.execute_opcode();
    }

    fn execute_opcode(&mut self) {
        match self.opcode {
            0x00E0 => self.op_00e0(),
            0x00EE => self.op_00ee(),

            0x1000..=0x1FFF => self.op_1nnn(),
            0x2000..=0x2FFF => self.op_2nnn(),
            0x3000..=0x3FFF => self.op_3xkk(),
            0x4000..=0x4FFF => self.op_4xkk(),

            0x5000..=0x5FFF => match self.opcode & 0x000F {
                0x0 => self.op_5xy0(),
                _ => self.op_unknow(),
            },

            0x6000..=0x6FFF => self.op_6xkk(),
            0x7000..=0x7FFF => self.op_7xkk(),

            0x8000..=0x8FFF => match self.opcode & 0x000F {
                0x0 => self.op_8xy0(),
                0x1 => self.op_8xy1(),
                0x2 => self.op_8xy2(),
                0x3 => self.op_8xy3(),
                0x4 => self.op_8xy4(),
                0x5 => self.op_8xy5(),
                0x6 => self.op_8xy6(),
                0x7 => self.op_8xy7(),
                0xE => self.op_8xye(),
                _ => self.op_unknow(),
            },

            0x9000..=0x9FFF => match self.opcode & 0x000F {
                0x0 => self.op_9xy0(),
                _ => self.op_unknow(),
            },

            0xA000..=0xAFFF => self.op_annn(),
            0xB000..=0xBFFF => self.op_bnnn(),
            0xC000..=0xCFFF => self.op_cxkk(),
            0xD000..=0xDFFF => self.op_dxyn(),

            0xE000..=0xEFFF => match self.opcode & 0x00FF {
                0xA1 => self.op_exa1(),
                0x9E => self.op_ex9e(),
                _ => self.op_unknow(),
            },

            0xF000..=0xFFFF => match self.opcode & 0x00FF {
                0x07 => self.op_fx07(),
                0x0A => self.op_fx0a(),
                0x15 => self.op_fx15(),
                0x18 => self.op_fx18(),
                0x1E => self.op_fx1e(),
                0x29 => self.op_fx29(),
                0x33 => self.op_fx33(),
                0x55 => self.op_fx55(),
                0x65 => self.op_fx65(),
                _ => self.op_unknow(),
            },

            _ => self.op_unknow(),
        }
    }

    fn rand_gen(&mut self) -> u8 {
        rng().random_range(0..255)
    }

    /// Clear screen
    fn op_00e0(&mut self) {
        self.video_frame_buffer.fill(0);
    }

    /// Return
    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// Jump to address
    fn op_1nnn(&mut self) {
        self.pc = self.opcode & 0x0FFF;
    }

    /// Call address
    fn op_2nnn(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;

        self.pc = self.opcode & 0x0FFF;
    }

    /// Skip next instruction if Vx == kk
    fn op_3xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        if self.registers[vx] == byte {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx != kk
    fn op_4xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        if self.registers[vx] != byte {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx == Vy
    fn op_5xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vx] == self.registers[vy] {
            self.pc += 2;
        }
    }

    /// Set Vx = kk
    fn op_6xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = byte;
    }

    /// Set Vx = Vx + kk
    fn op_7xkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = self.registers[vx].wrapping_add(byte);
    }

    /// Set Vx = Vy
    fn op_8xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] = self.registers[vy];
    }

    /// Set Vx = Vx OR Vy
    fn op_8xy1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] |= self.registers[vy];
    }

    /// Set Vx = Vx AND Vy
    fn op_8xy2(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] &= self.registers[vy];
    }

    /// Set Vx = Vx XOR Vy
    fn op_8xy3(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        self.registers[vx] ^= self.registers[vy];
    }

    /// Set Vx = Vx + Vy, set VF = carry
    fn op_8xy4(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        let sum = self.registers[vx] as u16 + self.registers[vy] as u16;

        if sum > 255 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] = (sum & 0xFF) as u8;
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow
    fn op_8xy5(&mut self) {
        let vx_index = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy_index = ((self.opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers[vx_index];
        let vy = self.registers[vy_index];

        if vx > vy {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx_index] = vx.wrapping_sub(vy);
    }

    /// Set Vx = Vx SHR 1
    fn op_8xy6(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[0xF] = self.registers[vx] & 0x1;

        self.registers[vx] >>= 1;
    }

    /// Set Vx = Vy - Vx, set VF = NOT borrow
    fn op_8xy7(&mut self) {
        let vx_index = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy_index = ((self.opcode & 0x00F0) >> 4) as usize;
        let vx = self.registers[vx_index];
        let vy = self.registers[vy_index];

        if vy > vx {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx_index] = vy.wrapping_sub(vx);
    }

    /// Set Vx = Vx SHL 1
    fn op_8xye(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;

        self.registers[vx] <<= 1;
    }

    /// Skip next instruction if Vx != Vy
    fn op_9xy0(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;

        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }

    /// Set I = nnn
    fn op_annn(&mut self) {
        self.index = self.opcode & 0x0FFF;
    }

    /// Jump to location nnn + V0
    fn op_bnnn(&mut self) {
        self.pc = self.registers[0] as u16 + (self.opcode & 0x0FFF);
    }

    /// Set Vx = random byte AND kk
    fn op_cxkk(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let byte = (self.opcode & 0x00FF) as u8;

        self.registers[vx] = self.rand_gen() & byte;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
    fn op_dxyn(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let vy = ((self.opcode & 0x00F0) >> 4) as usize;
        let height = (self.opcode & 0x000F) as u8;

        let x_pos = (self.registers[vx] as usize % SCREEN_WIDTH) as u16;
        let y_pos = (self.registers[vy] as usize % SCREEN_HEIGHT) as u16;

        self.registers[0xF] = 0;

        for row in 0..height as u16 {
            let sprite_row = self.memory[(self.index + row) as usize];

            for col in 0..8 {
                let sprite_pixel = sprite_row & (0x80 >> col);
                if sprite_pixel == 0 {
                    continue;
                }

                let x = (x_pos + col) as usize % SCREEN_WIDTH;
                let y = (y_pos + row) as usize % SCREEN_HEIGHT;
                let screen_pixel = &mut self.video_frame_buffer[y * SCREEN_WIDTH + x];

                if *screen_pixel == 1 {
                    self.registers[0xF] = 1;
                }

                *screen_pixel ^= 1;
            }
        }
    }

    /// Skip next instruction if key with the value of Vx is pressed
    fn op_ex9e(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        if self.keypad[self.registers[vx] as usize] == 1 {
            self.pc += 2;
        }
    }

    /// Skip next instruction if key with the value of Vx is not pressed
    fn op_exa1(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        if self.keypad[self.registers[vx] as usize] == 0 {
            self.pc += 2;
        }
    }

    /// Set Vx = delay timer value
    fn op_fx07(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.registers[vx] = self.delay_timer;
    }

    /// Wait for a key press, store the value of the key in Vx
    fn op_fx0a(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        if let Some(key) = self.keypad.iter().position(|&p| p == 1) {
            self.registers[vx] = key as u8;
            return;
        }

        self.pc -= 2;
    }

    /// Set delay timer = Vx
    fn op_fx15(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.delay_timer = self.registers[vx];
    }

    /// Set sound timer = Vx
    fn op_fx18(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.sound_timer = self.registers[vx];
    }

    /// Set I = I + Vx
    fn op_fx1e(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.index += self.registers[vx] as u16;
    }

    /// Set I = location of sprite for digit Vx
    fn op_fx29(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        self.index = FONT_SET_START_ADDRESS + (5 * self.registers[vx]) as u16;
    }

    /// Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn op_fx33(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;
        let mut value = self.registers[vx];

        self.memory[self.index as usize + 2] = value % 10;
        value /= 10;

        self.memory[self.index as usize + 1] = value % 10;
        value /= 10;

        self.memory[self.index as usize] = value % 10;
    }

    /// Store registers V0 through Vx in memory starting at location I
    fn op_fx55(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        for i in 0..=vx {
            self.memory[self.index as usize + i] = self.registers[i];
        }
    }

    /// Read registers V0 through Vx from memory starting at location I
    fn op_fx65(&mut self) {
        let vx = ((self.opcode & 0x0F00) >> 8) as usize;

        for i in 0..=vx {
            self.registers[i] = self.memory[self.index as usize + i];
        }
    }

    /// Unknow opcode
    fn op_unknow(&self) {
        eprintln!("Opcode {:04X} not implemented", self.opcode);
    }
}
