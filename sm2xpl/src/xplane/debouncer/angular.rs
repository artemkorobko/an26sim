use std::time::Duration;

use num_traits::{Float, NumCast};

use super::{generic::Debouncer, linear::LinearDebouncer};

#[derive(Default)]
pub struct AngularDebouncer<T: Default> {
    linear: LinearDebouncer<T>,
}

impl<T: Default + Float + std::fmt::Debug> AngularDebouncer<T> {
    pub fn new(barrier: T) -> Self {
        let linear = LinearDebouncer::new(barrier);
        Self { linear }
    }

    fn on_edge(&self) -> bool {
        self.on_upper_adge() || self.on_lower_edge()
    }

    fn on_upper_adge(&self) -> bool {
        self.linear.value >= NumCast::from(350.0).unwrap()
            && self.linear.value <= NumCast::from(360.0).unwrap()
    }

    fn on_lower_edge(&self) -> bool {
        self.linear.value >= NumCast::from(0.0).unwrap()
            && self.linear.value <= NumCast::from(10.0).unwrap()
    }

    fn calc_edge_step(&self, target: T) -> T {
        let diff = target - self.linear.value;
        let step = if diff > self.linear.barrier {
            if diff.is_sign_negative() {
                self.linear.value - diff.abs()
            } else {
                diff - self.linear.value
            }
        } else {
            target - self.linear.value
        };

        step
    }

    fn is_bouncing_on_edge(&self, target: T) -> bool {
        target.abs_sub(self.linear.value) >= NumCast::from(360.0).unwrap()
    }
}

impl<T> Debouncer<T> for AngularDebouncer<T>
where
    T: Default + Float + std::fmt::Debug,
{
    fn debounce(&mut self, target: T, delta: &Duration) -> T {
        if self.on_edge() {
            if self.is_bouncing_on_edge(target) {
                let mut value = self.linear.debounce(target, delta);
                let max = NumCast::from(359.99).unwrap();
                if value > max {
                    value = value - max;
                    self.linear.value = value;
                }

                value
            } else {
                let step = self.calc_edge_step(target);
                let value = self.linear.update(target);
                self.linear.step = step;
                value
            }
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

#[cfg(test)]
mod test {
    use float_eq::assert_float_eq;

    use crate::xplane::debouncer::MAX_BOUNCE_TIME;

    use super::*;

    const BARRIER: f64 = 10.0;
    const PRECISION: f64 = 0.01;

    #[test]
    fn should_integrate_not_on_edge_during_the_timeout() {
        let mut debouncer = AngularDebouncer::new(BARRIER);

        let value = debouncer.debounce(5.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
        let value = debouncer.debounce(10.0, &Duration::ZERO);
        assert_float_eq!(value, 10.0, abs <= PRECISION);
        let value = debouncer.debounce(15.0, &Duration::ZERO);
        assert_float_eq!(value, 15.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 20.0, abs <= PRECISION);
        let value = debouncer.debounce(25.0, &Duration::ZERO);
        assert_float_eq!(value, 25.0, abs <= PRECISION);
        let value = debouncer.debounce(20.0, &Duration::ZERO);
        assert_float_eq!(value, 20.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 15.0, abs <= PRECISION);
        let value = debouncer.debounce(10.0, &Duration::ZERO);
        assert_float_eq!(value, 10.0, abs <= PRECISION);
    }

    #[test]
    fn should_assign_after_the_timeout() {
        let mut debouncer = AngularDebouncer::new(BARRIER);

        let value = debouncer.debounce(5.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
        let value = debouncer.debounce(10.0, &Duration::ZERO);
        assert_float_eq!(value, 10.0, abs <= PRECISION);
        let value = debouncer.debounce(15.0, &Duration::ZERO);
        assert_float_eq!(value, 15.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 20.0, abs <= PRECISION);
        let value = debouncer.debounce(50.0, &MAX_BOUNCE_TIME);
        assert_float_eq!(value, 50.0, abs <= PRECISION);
    }

    #[test]
    fn should_not_debounce_on_edges() {
        let mut debouncer = AngularDebouncer::new(BARRIER);
        debouncer.linear.value = 345.0;

        let value = debouncer.debounce(350.0, &Duration::ZERO);
        assert_float_eq!(value, 350.0, abs <= PRECISION);
        let value = debouncer.debounce(355.0, &Duration::ZERO);
        assert_float_eq!(value, 355.0, abs <= PRECISION);
        let value = debouncer.debounce(0.0, &Duration::ZERO);
        assert_float_eq!(value, 0.0, abs <= PRECISION);
        let value = debouncer.debounce(5.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
        let value = debouncer.debounce(10.0, &Duration::ZERO);
        assert_float_eq!(value, 10.0, abs <= PRECISION);
        let value = debouncer.debounce(5.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
        let value = debouncer.debounce(0.0, &Duration::ZERO);
        assert_float_eq!(value, 0.0, abs <= PRECISION);
        let value = debouncer.debounce(355.0, &Duration::ZERO);
        assert_float_eq!(value, 355.0, abs <= PRECISION);
        let value = debouncer.debounce(350.0, &Duration::ZERO);
        assert_float_eq!(value, 350.0, abs <= PRECISION);
    }

    #[test]
    fn should_intergrate_on_edges() {
        let mut debouncer = AngularDebouncer::new(BARRIER);
        debouncer.linear.value = 345.0;

        let value = debouncer.debounce(350.0, &Duration::ZERO);
        assert_float_eq!(value, 350.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 355.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 0.0, abs <= PRECISION);
        let value = debouncer.debounce(1000.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
        let value = debouncer.debounce(5.0, &Duration::ZERO);
        assert_float_eq!(value, 5.0, abs <= PRECISION);
    }
}
