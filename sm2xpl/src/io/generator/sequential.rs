use std::time::Duration;

use num_traits::PrimInt;

use crate::common::timer::DeltaCounter;

use super::generic::Generator;

#[derive(Copy, Clone)]
enum Operation {
    Increase,
    Decrease,
}

#[derive(Copy, Clone)]
pub struct SequentialGenerator<T> {
    param: T,
    step: T,
    min: T,
    max: T,
    operation: Operation,
    timer: DeltaCounter,
}

impl<T: PrimInt> SequentialGenerator<T> {
    pub fn new(step: T, min: T, max: T, timeout: Duration) -> Self {
        Self {
            param: T::zero(),
            step,
            min,
            max,
            operation: Operation::Increase,
            timer: DeltaCounter::deferred(timeout),
        }
    }

    pub fn min(&self) -> T {
        self.min
    }

    pub fn max(&self) -> T {
        self.max
    }

    fn reverse_operation(&mut self) {
        match self.operation {
            Operation::Increase => self.operation = Operation::Decrease,
            Operation::Decrease => self.operation = Operation::Increase,
        }
    }
}

impl<T: PrimInt> Generator<T> for SequentialGenerator<T> {
    fn generate(&mut self, delta: &Duration) -> T {
        if self.timer.count(&delta).is_elapsed() {
            let result = match self.operation {
                Operation::Increase => self.param.checked_add(&self.step),
                Operation::Decrease => self.param.checked_sub(&self.step),
            };

            if let Some(param) = result {
                if param < self.min || param > self.max {
                    self.reverse_operation();
                } else {
                    self.param = param;
                }
            } else {
                self.reverse_operation();
            }
        }

        self.param
    }

    fn reset(&mut self, param: T) {
        self.param = param;
    }
}

mod test {
    use super::*;

    const STEP: i32 = 3;
    const MIN: i32 = 0;
    const MAX: i32 = 10;

    #[test]
    fn should_generate_in_all_directions() {
        let timeout = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(STEP, MIN, MAX, timeout);

        assert_eq!(generator.generate(&timeout), 3);
        assert_eq!(generator.generate(&timeout), 6);
        assert_eq!(generator.generate(&timeout), 9);
        assert_eq!(generator.generate(&timeout), 9);
        assert_eq!(generator.generate(&timeout), 6);
        assert_eq!(generator.generate(&timeout), 3);
        assert_eq!(generator.generate(&timeout), 0);
    }

    #[test]
    fn should_generate_only_within_interval() {
        let mut generator = SequentialGenerator::new(STEP, MIN, MAX, Duration::from_secs(1));

        assert_eq!(generator.generate(&Duration::from_millis(500)), 0);
        assert_eq!(generator.generate(&Duration::from_millis(400)), 0);
        assert_eq!(generator.generate(&Duration::from_millis(200)), 3);
    }

    #[test]
    fn should_reset_parameter() {
        let duration = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(STEP, MIN, MAX, duration);

        assert_eq!(generator.generate(&duration), 3);
        assert_eq!(generator.generate(&duration), 6);
        generator.reset(1);
        assert_eq!(generator.generate(&duration), 4);
        assert_eq!(generator.generate(&duration), 7);
    }
}
