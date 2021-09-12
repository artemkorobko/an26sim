use std::time::Duration;

use crate::common::timer::{DeltaCounter, Elapsed};

use super::generic::Generator;

enum Operation {
    Increase,
    Decrease,
}

enum GeneratorResult {
    Ok(i16),
    Overflow,
}

pub struct SequentialGenerator {
    value: i16,
    step: i16,
    time: DeltaCounter,
    operation: Operation,
}

impl SequentialGenerator {
    pub fn new(default: i16, step: i16, delay: Duration) -> Self {
        Self {
            value: default,
            step,
            time: DeltaCounter::deferred(delay),
            operation: Operation::Increase,
        }
    }

    fn generate_value(&self) -> GeneratorResult {
        let (value, overflow) = match self.operation {
            Operation::Increase => self.value.overflowing_add(self.step),
            Operation::Decrease => self.value.overflowing_sub(self.step),
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

impl Generator for SequentialGenerator {
    fn generate(&mut self, delta: Duration) -> i16 {
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
