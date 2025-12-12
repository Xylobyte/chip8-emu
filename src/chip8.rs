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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; 16],
    pub video_frame_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub opcode: u16,
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
            file.read(&mut buf).unwrap();
        }

        for i in 0..buf.len() {
            self.memory[START_ADDRESS as usize + i] = buf[i];
        }
    }

    pub fn rand_gen(&mut self) -> u8 {
        rng().random_range(0..255)
    }
}
