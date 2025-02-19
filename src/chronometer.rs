use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Chronometer {
    frame_duration: Duration,
    last_update: Instant,
}

impl Chronometer {
    pub fn new(target_fps: u32) -> Chronometer {
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
        let last_update = Instant::now();

        return Chronometer {
            frame_duration,
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
