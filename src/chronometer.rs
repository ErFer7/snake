use std::{thread, time::{Duration, Instant}};

pub struct Chronometer {
    target_fps: u32,
    frame_duration: Duration,
    start_time: Instant,
    last_update: Instant,
}

impl Chronometer {
    pub fn new(target_fps: u32) -> Chronometer {
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
        let start_time = Instant::now();
        let last_update = Instant::now();

        return Chronometer {
            target_fps,
            frame_duration,
            start_time,
            last_update,
        };
    }

    /// Returns false if the frame should be skipped.
    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now - self.last_update;

        if elapsed >= self.frame_duration {
            self.last_update = now;
            return true;
        }

        thread::sleep(Duration::from_millis(1));
        return false;
    }
}