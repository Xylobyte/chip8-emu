use crate::chip8::Chip8;
use crate::{CPU_DT, TIMER_DT};
use std::thread;
use std::time::{Duration, Instant};

pub struct Scheduler {
    last_time: Instant,
    cpu_time: f64,
    timer_time: f64,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
            cpu_time: 0.0,
            timer_time: 0.0,
        }
    }

    pub fn tick<F>(&mut self, mut f: F, chip8: &mut Chip8)
    where
        F: FnMut(&mut Chip8, bool),
    {
        let now = Instant::now();
        let dt = (now - self.last_time).as_secs_f64().min(0.25);
        self.last_time = now;

        self.cpu_time += dt;
        self.timer_time += dt;

        loop {
            let next_cpu = CPU_DT - self.cpu_time;
            let next_timer = TIMER_DT - self.timer_time;

            if next_cpu <= 0.0 {
                f(chip8, false);
                self.cpu_time -= CPU_DT;
            } else if next_timer <= 0.0 {
                f(chip8, true);
                self.timer_time -= TIMER_DT;
            } else {
                break;
            }
        }

        let sleep = CPU_DT.min(TIMER_DT).min(self.cpu_time).min(self.timer_time);

        if sleep > 0.0 {
            thread::sleep(Duration::from_secs_f64(sleep));
        }
    }
}
