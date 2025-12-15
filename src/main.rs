mod emu;
mod chip8;
mod scheduler;

use crate::emu::Emulator;

const CPU_HZ: f64 = 800.0;
const TIMER_HZ: f64 = 60.0;

const CPU_DT: f64 = 1.0 / CPU_HZ;
const TIMER_DT: f64 = 1.0 / TIMER_HZ;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SCALE: usize = 10;

const ROM_FILENAME: &str = "roms/astro_dodge.ch8";

fn main() {
    let mut emu = Emulator::new();
    emu.run();
}
