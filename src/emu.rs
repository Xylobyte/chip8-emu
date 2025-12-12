use crate::chip8::Chip8;
use crate::{ROM_FILENAME, SCREEN_HEIGHT, SCREEN_SCALE, SCREEN_WIDTH};
use minifb::{Key, Window, WindowOptions};

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

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            chip8.video_frame_buffer[1] = 1;
            chip8.video_frame_buffer[2 + SCREEN_WIDTH] = 1;
            chip8.video_frame_buffer[3] = 1;
            chip8.video_frame_buffer[5] = 1;


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
