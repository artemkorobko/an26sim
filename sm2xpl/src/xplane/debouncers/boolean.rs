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
            if self.can_integrate() {
                self.integrate(delta)
            } else {
                self.assign(target)
            }
        } else {
            self.update(target)
        }
    }

    fn integrate(&mut self, delta: &Duration) -> bool {
        self.integration_time += *delta;
        self.value
    }

    fn assign(&mut self, target: bool) -> bool {
        self.integration_time = Duration::ZERO;
        self.value = target;
        self.value
    }
}
