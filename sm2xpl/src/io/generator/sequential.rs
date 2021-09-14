use std::time::Duration;

use crate::common::timer::{DeltaCounter, Elapsed};

use super::{generator::Generator, parameter::Parameter};

enum Operation {
    Increase,
    Decrease,
}

pub struct SequentialGenerator<T: Parameter> {
    value: T,
    step: T,
    time: DeltaCounter,
    operation: Operation,
}

impl<T: Parameter> From<T> for SequentialGenerator<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            step: T::zero(),
            time: DeltaCounter::default(),
            operation: Operation::Increase,
        }
    }
}

impl<T: Parameter + Copy> SequentialGenerator<T> {
    pub fn with_step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    pub fn deferred(mut self, delay: Duration) -> Self {
        self.time = DeltaCounter::deferred(delay);
        self
    }

    pub fn increase(mut self) -> Self {
        self.operation = Operation::Increase;
        self
    }

    pub fn decrease(mut self) -> Self {
        self.operation = Operation::Decrease;
        self
    }

    fn generate_param(&self) -> Option<T> {
        match self.operation {
            Operation::Increase => self.value.increase(self.step),
            Operation::Decrease => self.value.decrease(self.step),
        }
    }

    fn reverse_operation(&self) -> Operation {
        match self.operation {
            Operation::Increase => Operation::Decrease,
            Operation::Decrease => Operation::Increase,
        }
    }
}

impl<T: Parameter + Copy> Generator for SequentialGenerator<T> {
    fn generate(&mut self, delta: Duration) -> Vec<u8> {
        match self.time.count(delta.clone()) {
            Elapsed::Yes(diff) => {
                self.time.count(diff);
                match self.generate_param() {
                    Some(value) => {
                        self.value = value;
                        self.value.to_be_bytes_vec()
                    }
                    None => {
                        self.operation = self.reverse_operation();
                        self.generate(delta)
                    }
                }
            }
            Elapsed::No => self.value.to_be_bytes_vec(),
        }
    }

    fn size_bytes(&self) -> usize {
        self.value.size_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_immediate_value() {
        let default = 100i16;
        let step = 1;
        let mut generator = SequentialGenerator::from(default).with_step(step);

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, (default + step).to_be_bytes());

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, (default + step + step).to_be_bytes());
    }

    #[test]
    fn generate_dereffed_value() {
        let default = 50i16;
        let step = 10;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::from(default)
            .with_step(step)
            .deferred(delay);

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, default.to_be_bytes());

        let value = generator.generate(delay);
        assert_eq!(value, (default + step).to_be_bytes());
    }

    #[test]
    fn reverse_direction_on_max_overflow() {
        let step = 5;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::from(i16::MAX - step)
            .with_step(step)
            .deferred(delay);

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, (i16::MAX - step).to_be_bytes());

        let value = generator.generate(delay.clone());
        assert_eq!(value, i16::MAX.to_be_bytes());

        let value = generator.generate(delay);
        assert_eq!(value, (i16::MAX - step).to_be_bytes());
    }

    #[test]
    fn reverse_direction_on_min_overflow() {
        let step = 8;
        let delay = Duration::from_secs(1);
        let mut generator = SequentialGenerator::from(i16::MIN + step)
            .with_step(step)
            .deferred(delay)
            .decrease();

        let value = generator.generate(Duration::ZERO);
        assert_eq!(value, (i16::MIN + step).to_be_bytes());

        let value = generator.generate(delay.clone());
        assert_eq!(value, i16::MIN.to_be_bytes());

        let value = generator.generate(delay);
        assert_eq!(value, (i16::MIN + step).to_be_bytes());
    }

    #[test]
    fn return_size_bytes() {
        let generator = SequentialGenerator::from(0u16);

        let size = generator.size_bytes();

        assert_eq!(size, 2);
    }
}
