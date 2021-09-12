use std::time::Duration;

use num_traits::ops::overflowing::{OverflowingAdd, OverflowingSub};

use crate::common::timer::{DeltaCounter, Elapsed};

use super::generic::Generator;

enum Operation {
    Increase,
    Decrease,
}

enum GeneratorResult<T> {
    Ok(T),
    Overflow,
}

pub struct SequentialGenerator<T> {
    value: T,
    step: T,
    time: DeltaCounter,
    operation: Operation,
}

impl<T: OverflowingAdd + OverflowingSub> SequentialGenerator<T> {
    pub fn new(default: T, step: T, delay: Duration) -> Self {
        Self {
            value: default,
            step,
            time: DeltaCounter::deferred(delay),
            operation: Operation::Increase,
        }
    }

    fn generate_value(&self) -> GeneratorResult<T> {
        let (value, overflow) = match self.operation {
            Operation::Increase => self.value.overflowing_add(&self.step),
            Operation::Decrease => self.value.overflowing_sub(&self.step),
        };

        match overflow {
            true => GeneratorResult::Overflow,
            false => GeneratorResult::Ok(value),
        }
    }

    fn reverse_operation(&self) -> Operation {
        match self.operation {
            Operation::Increase => Operation::Decrease,
            Operation::Decrease => Operation::Increase,
        }
    }
}

impl<T: Copy + OverflowingAdd + OverflowingSub> Generator<T> for SequentialGenerator<T> {
    fn generate(&mut self, delta: Duration) -> T {
        match self.time.count(delta.clone()) {
            Elapsed::Yes(diff) => {
                self.time.count(diff);
                match self.generate_value() {
                    GeneratorResult::Ok(value) => {
                        self.value = value;
                        self.value
                    }
                    GeneratorResult::Overflow => {
                        self.operation = self.reverse_operation();
                        self.generate(delta)
                    }
                }
            }
            Elapsed::No => self.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_value() {
        let default = 50;
        let step = 10;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(default, step, delay);

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, default);

        let value = generator.generate(delay);
        assert_eq!(value, default + step);
    }

    #[test]
    fn should_reverse_direction_on_max_overflow() {
        let step = 5;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(i16::MAX - step, step, delay);

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, i16::MAX - step);

        let value = generator.generate(delay.clone());
        assert_eq!(value, i16::MAX);

        let value = generator.generate(delay);
        assert_eq!(value, i16::MAX - step);
    }

    #[test]
    fn should_reverse_direction_on_min_overflow() {
        let step = 8;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(i16::MIN + step, step, delay);
        generator.operation = Operation::Decrease;

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, i16::MIN + step);

        let value = generator.generate(delay.clone());
        assert_eq!(value, i16::MIN);

        let value = generator.generate(delay);
        assert_eq!(value, i16::MIN + step);
    }
}
