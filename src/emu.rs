use crate::chip8::Chip8;
use crate::scheduler::Scheduler;
use crate::{ROM_FILENAME, SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH};
use minifb::{Key, KeyRepeat, Window, WindowOptions};

pub struct Emulator {
    window: Window,
    screen: [u32; SCREEN_WIDTH * SCREEN_HEIGHT * SCREEN_SCALE * SCREEN_SCALE],
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            window: Window::new(
                "CHIP-8",
                SCREEN_WIDTH * SCREEN_SCALE,
                SCREEN_HEIGHT * SCREEN_SCALE,
                WindowOptions::default(),
            )
                .unwrap(),
            screen: [0; SCREEN_WIDTH * SCREEN_HEIGHT * SCREEN_SCALE * SCREEN_SCALE],
        }
    }

    pub fn run(&mut self) {
        let mut chip8 = Chip8::new();
        chip8.load_rom(ROM_FILENAME);

        let mut scheduler = Scheduler::new();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.detect_keyboard(&mut chip8);

            scheduler.tick(
                |chip8, is_timer_step| {
                    if is_timer_step {
                        chip8.update_timers();
                    } else {
                        chip8.execute_cycle();
                    }
                },
                &mut chip8,
            );

            self.scale_fb(&chip8.video_frame_buffer);
            self.window
                .update_with_buffer(
                    &self.screen,
                    SCREEN_WIDTH * SCREEN_SCALE,
                    SCREEN_HEIGHT * SCREEN_SCALE,
                )
                .unwrap();
        }
    }

    fn detect_keyboard(&self, chip8: &mut Chip8) {
        for k in self.window.get_keys_pressed(KeyRepeat::No) {
            if let Some(key) = Self::map_key(&k) {
                chip8.keypad[key] = 1;
            }
        }

        for k in self.window.get_keys_released() {
            if let Some(key) = Self::map_key(&k) {
                chip8.keypad[key] = 0;
            }
        }
    }

    fn map_key(key: &Key) -> Option<usize> {
        match key {
            Key::X => Some(0),
            Key::Key1 => Some(1),
            Key::Key2 => Some(2),
            Key::Key3 => Some(3),
            Key::Q => Some(4),
            Key::W => Some(5),
            Key::E => Some(6),
            Key::A => Some(7),
            Key::S => Some(8),
            Key::D => Some(9),
            Key::Z => Some(0xA),
            Key::C => Some(0xB),
            Key::Key4 => Some(0xC),
            Key::R => Some(0xD),
            Key::F => Some(0xE),
            Key::V => Some(0xF),
            _ => None,
        }
    }

    fn scale_fb(&mut self, fb: &[u8; SCREEN_WIDTH * SCREEN_HEIGHT]) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let pixel_on = fb[y * SCREEN_WIDTH + x] != 0;
                let color = if pixel_on { 0xFFFFFF } else { 0x000000 };

                for sy in 0..SCREEN_SCALE {
                    for sx in 0..SCREEN_SCALE {
                        let dst_y = y * SCREEN_SCALE + sy;
                        let dst_x = x * SCREEN_SCALE + sx;
                        self.screen[dst_y * SCREEN_WIDTH * SCREEN_SCALE + dst_x] = color;
                    }
                }
            }
        }
    }
}
