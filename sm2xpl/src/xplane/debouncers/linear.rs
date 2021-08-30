use std::time::Duration;

use num_traits::Float;

use super::{generic::Debouncer, MAX_BOUNCE_TIME};

#[derive(Default)]
pub struct LinearDebouncer<T: Default> {
    pub barrier: T,
    pub value: T,
    pub step: T,
    pub integration_time: Duration,
}

impl<T: Default> LinearDebouncer<T> {
    pub fn new(barrier: T) -> Self {
        Self {
            barrier,
            ..Default::default()
        }
    }
}

impl<T> LinearDebouncer<T>
where
    T: Default + Float + std::fmt::Debug,
{
    pub fn update(&mut self, target: T) -> T {
        self.integration_time = Duration::ZERO;
        self.step = target - self.value;
        self.value = target;
        self.value
    }

    fn is_bouncing(&self, target: T) -> bool {
        target.abs_sub(self.value) >= self.barrier
    }

    fn can_integrate(&self) -> bool {
        self.integration_time < MAX_BOUNCE_TIME
    }
}

impl<T> Debouncer<T> for LinearDebouncer<T>
where
    T: Default + Float + std::fmt::Debug,
{
    fn debounce(&mut self, target: T, delta: &Duration) -> T {
        if self.is_bouncing(target) {
            println!(
                "bounced: V:{:?}, T:{:?}, B:{:?}",
                self.value, target, self.barrier
            );
            if self.can_integrate() {
                self.integrate(delta)
            } else {
                self.assign(target)
            }
        } else {
            self.update(target)
        }
    }

    fn integrate(&mut self, delta: &Duration) -> T {
        self.integration_time += *delta;
        self.value = self.value + self.step;
        println!("integrate: V:{:?}, S:{:?}", self.value, self.step);
        self.value
    }

    fn assign(&mut self, target: T) -> T {
        self.integration_time = Duration::ZERO;
        self.step = T::zero();
        self.value = target;
        self.value
    }
}
