use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Elapsed {
    Yes(Duration),
    No,
}

impl Elapsed {
    pub fn is_elapsed(&self) -> bool {
        match self {
            Elapsed::Yes(_) => true,
            Elapsed::No => false,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct DeltaCounter {
    delay: Duration,
    timer: Duration,
}

impl DeltaCounter {
    pub fn immediate(delay: Duration) -> Self {
        Self {
            delay,
            timer: delay,
        }
    }

    pub fn deferred(delay: Duration) -> Self {
        Self {
            delay,
            timer: Duration::ZERO,
        }
    }

    pub fn delay(&self) -> Duration {
        self.delay
    }

    pub fn count(&mut self, delta: Duration) -> Elapsed {
        self.timer += delta;
        if self.timer >= self.delay {
            let diff = self.timer - self.delay;
            self.timer = Duration::ZERO;
            Elapsed::Yes(diff)
        } else {
            Elapsed::No
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_default_counter() {
        let mut counter = DeltaCounter::default();

        let result = counter.count(Duration::ZERO);

        assert_eq!(result, Elapsed::Yes(Duration::ZERO));
    }

    #[test]
    fn count_immediate_time() {
        let mut counter = DeltaCounter::immediate(Duration::from_secs(1));

        let result = counter.count(Duration::from_millis(500));
        assert!(result.is_elapsed());
        assert_eq!(result, Elapsed::Yes(Duration::from_millis(500)));

        let result = counter.count(Duration::from_millis(900));
        assert!(!result.is_elapsed());
        assert_eq!(result, Elapsed::No);

        let result = counter.count(Duration::from_millis(200));
        assert!(result.is_elapsed());
        assert_eq!(result, Elapsed::Yes(Duration::from_millis(100)));
    }

    #[test]
    fn count_deferred_time() {
        let mut counter = DeltaCounter::deferred(Duration::from_secs(1));

        let result = counter.count(Duration::from_millis(500));
        assert!(!result.is_elapsed());
        assert_eq!(result, Elapsed::No);

        let result = counter.count(Duration::from_millis(900));
        assert!(result.is_elapsed());
        assert_eq!(result, Elapsed::Yes(Duration::from_millis(400)));
    }
}
