use std::time::Duration;

use num_traits::PrimInt;
use rand::{distributions::uniform::SampleUniform, Rng};

use super::generic::Generator;

pub struct BouncedGenerator<T> {
    generator: Box<dyn Generator<T>>,
    bounce_on: usize,
    sequences: usize,
}

impl<T> BouncedGenerator<T> {
    pub fn new(generator: Box<dyn Generator<T>>, bounce_on: usize) -> Self {
        Self {
            generator,
            bounce_on,
            sequences: 0,
        }
    }

    fn count_sequence(&mut self) {
        self.sequences += 1;
    }

    fn reset_sequeences(&mut self) {
        self.sequences = 0;
    }

    fn should_bounce(&self) -> bool {
        self.sequences == self.bounce_on
    }
}

impl<T: PrimInt + SampleUniform> Generator<T> for BouncedGenerator<T> {
    fn generate(&mut self, delta: Duration) -> T {
        let value = self.generator.generate(delta);
        self.count_sequence();
        if self.should_bounce() {
            self.reset_sequeences();
            rand::thread_rng().gen_range(T::min_value()..T::max_value())
        } else {
            value
        }
    }
}

#[cfg(test)]
mod test {
    use crate::io::generator::sequential::SequentialGenerator;

    use super::*;

    #[test]
    fn should_bounce_every_3_iteration() {
        let default = 100;
        let step = 1;
        let delay = Duration::from_secs(1);
        let sequential = SequentialGenerator::new(default, step, delay.clone());

        let mut generator = BouncedGenerator::new(Box::new(sequential), 3);

        let mut expected = default + step;
        assert_eq!(generator.generate(delay.clone()), expected);
        expected += step;
        assert_eq!(generator.generate(delay.clone()), expected);
        expected += step;
        assert_ne!(generator.generate(delay.clone()), expected);
        expected += step;
        assert_eq!(generator.generate(delay.clone()), expected);
        expected += step;
        assert_eq!(generator.generate(delay.clone()), expected);
    }
}
