use std::{marker::PhantomData, time::Duration};

use super::{generator::Generator, parameter::Parameter};

pub struct BouncedGenerator<T: Parameter> {
    generator: Box<dyn Generator>,
    bounce_on: usize,
    sequences: usize,
    phantom: PhantomData<T>,
}

impl<T: Parameter> BouncedGenerator<T> {
    pub fn bounce(generator: Box<dyn Generator>) -> Self {
        Self {
            generator,
            bounce_on: 0,
            sequences: 0,
            phantom: PhantomData,
        }
    }

    pub fn every(mut self, sequence: usize) -> Self {
        self.bounce_on = sequence;
        self
    }

    fn count_sequence(&mut self) {
        self.sequences += 1;
    }

    fn reset_sequences(&mut self) {
        self.sequences = 0;
    }

    fn should_bounce(&self) -> bool {
        self.sequences == self.bounce_on
    }
}

impl<T: Parameter> Generator for BouncedGenerator<T> {
    fn generate(&mut self, delta: Duration) -> Vec<u8> {
        let value = self.generator.generate(delta);
        self.count_sequence();
        if self.should_bounce() {
            self.reset_sequences();
            T::random().to_be_bytes_vec()
        } else {
            value
        }
    }

    fn size_bytes(&self) -> usize {
        self.generator.size_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::io::generator::sequential::SequentialGenerator;

    use super::*;

    #[test]
    fn bounce_every_3_iteration() {
        let default = 100u32;
        let step = 1;
        let delay = Duration::from_secs(1);
        let sequential = SequentialGenerator::from(default).with_step(step);
        let mut generator = BouncedGenerator::<u32>::bounce(Box::new(sequential)).every(3);

        let mut expected = default + step;
        let bytes = generator.generate(delay.clone());
        assert_eq!(bytes, expected.to_be_bytes());

        expected += step;
        let bytes = generator.generate(delay.clone());
        assert_eq!(bytes, expected.to_be_bytes());

        expected += step;
        let bytes = generator.generate(delay.clone());
        assert_ne!(bytes, expected.to_be_bytes());

        expected += step;
        let bytes = generator.generate(delay.clone());
        assert_eq!(bytes, expected.to_be_bytes());

        expected += step;
        let bytes = generator.generate(delay.clone());
        assert_eq!(bytes, expected.to_be_bytes());

        expected += step;
        let bytes = generator.generate(delay.clone());
        assert_ne!(bytes, expected.to_be_bytes());
    }

    #[test]
    fn return_size_bytes() {
        let sequential = SequentialGenerator::from(0u32).with_step(0u32);
        let generator = BouncedGenerator::<u32>::bounce(Box::new(sequential));

        let size = generator.size_bytes();

        assert_eq!(size, 4);
    }
}
