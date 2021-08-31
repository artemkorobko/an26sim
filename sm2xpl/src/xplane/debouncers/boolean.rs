use std::time::Duration;

use super::{generic::Debouncer, MAX_BOUNCE_TIME};

#[derive(Default)]
pub struct BooleanDebouncer {
    value: bool,
    integration_time: Duration,
}

impl BooleanDebouncer {
    pub fn update(&mut self, target: bool) -> bool {
        self.integration_time = Duration::ZERO;
        self.value = target;
        self.value
    }

    fn is_bouncing(&self, target: bool) -> bool {
        self.value != target
    }

    fn can_integrate(&mut self) -> bool {
        self.integration_time < MAX_BOUNCE_TIME
    }
}

impl Debouncer<bool> for BooleanDebouncer {
    fn debounce(&mut self, target: bool, delta: &Duration) -> bool {
        if self.is_bouncing(target) {
            self.integration_time += *delta;
            if self.can_integrate() {
                self.integrate(delta)
            } else {
                self.assign(target)
            }
        } else {
            self.update(target)
        }
    }

    fn integrate(&mut self, _: &Duration) -> bool {
        self.value
    }

    fn assign(&mut self, target: bool) -> bool {
        self.integration_time = Duration::ZERO;
        self.value = target;
        self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_integrate_during_the_timeout() {
        let mut debouncer = BooleanDebouncer::default();

        assert_eq!(debouncer.debounce(true, &Duration::ZERO), false);
        assert_eq!(debouncer.debounce(false, &Duration::ZERO), false);
        assert_eq!(debouncer.debounce(true, &Duration::ZERO), false);
        assert_eq!(debouncer.debounce(false, &Duration::ZERO), false);
    }

    #[test]
    fn should_assign_after_the_timeout() {
        let mut debouncer = BooleanDebouncer::default();

        assert_eq!(debouncer.debounce(true, &Duration::ZERO), false);
        assert_eq!(debouncer.debounce(true, &MAX_BOUNCE_TIME), true);
        assert_eq!(debouncer.debounce(false, &Duration::ZERO), true);
    }
}
