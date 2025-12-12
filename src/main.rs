mod emu;
mod chip8;

use crate::emu::Emulator;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SCALE: usize = 10;

fn main() {
    let mut emu = Emulator::new();
    emu.run();
}
