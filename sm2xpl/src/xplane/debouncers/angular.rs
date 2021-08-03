use std::time::Duration;

use num_traits::{Float, NumCast};

use super::{generic::Debouncer, linear::LinearDebouncer};

pub struct AngularDebouncer<T: Default> {
    linear: LinearDebouncer<T>,
}

impl<T: Default> AngularDebouncer<T> {
    pub fn new(barrier: T) -> Self {
        let linear = LinearDebouncer::new(barrier);
        Self { linear }
    }
}

impl<T> Debouncer<T> for AngularDebouncer<T>
where
    T: Default + Float + std::fmt::Debug,
{
    fn debounce(&mut self, target: T, delta: &Duration) -> T {
        if target > NumCast::from(350.0).unwrap() && target < NumCast::from(360.0).unwrap() {
            let step = self.linear.step;
            let value = self.linear.update(target);
            self.linear.step = step;
            value
        } else {
            self.linear.debounce(target, delta)
        }
    }

    fn integrate(&mut self, delta: &Duration) -> T {
        self.linear.integrate(delta)
    }

    fn assign(&mut self, target: T) -> T {
        self.linear.assign(target)
    }
}
