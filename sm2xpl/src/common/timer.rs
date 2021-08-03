use std::time::Duration;

#[derive(Copy, Clone)]
pub struct TimeCounter {
    delay: Duration,
    timer: Duration,
}

impl TimeCounter {
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            timer: delay,
        }
    }

    pub fn count(&mut self, delta: &Duration) -> Option<Duration> {
        self.timer += *delta;
        if self.timer >= self.delay {
            let diff = self.timer - self.delay;
            self.timer = Duration::ZERO;
            Some(diff)
        } else {
            None
        }
    }
}
