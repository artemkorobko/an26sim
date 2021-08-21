use std::time::Duration;

use num_traits::{NumCast, PrimInt};
use rand::{distributions::uniform::SampleUniform, Rng};

use super::{generic::Generator, sequential::SequentialGenerator};

#[derive(Copy, Clone)]
pub struct BouncedGenerator<T> {
    gen: SequentialGenerator<T>,
    sequences: usize,
    debounced_sequences: usize,
}

impl<T: PrimInt + SampleUniform> BouncedGenerator<T> {
    pub fn new(step: T, min: T, max: T, dbs: usize, timeout: Duration) -> Self {
        Self {
            gen: SequentialGenerator::new(step, min, max, timeout),
            sequences: 0,
            debounced_sequences: dbs,
        }
    }

    fn should_bounce(&mut self) -> bool {
        self.sequences += 1;
        self.sequences == self.debounced_sequences
    }

    fn bounce(&self, param: T) -> T {
        if self.gen.max() - self.gen.min() >= NumCast::from(3).unwrap() {
            let mut result = param;
            while result == param {
                result = rand::thread_rng().gen_range(self.gen.min()..self.gen.max())
            }

            result
        } else {
            param
        }
    }
}

impl<T: PrimInt + SampleUniform> Generator<T> for BouncedGenerator<T> {
    fn generate(&mut self, delta: &Duration) -> T {
        let param = self.gen.generate(delta);
        if self.should_bounce() {
            self.sequences = 0;
            self.bounce(param)
        } else {
            param
        }
    }

    fn reset(&mut self, param: T) {
        self.gen.reset(param);
    }
}

mod test {
    use super::*;

    #[test]
    fn should_bounce_every_3_iteration() {
        let min = 0;
        let max = 100;
        let duration = Duration::from_secs(1);
        let mut gen = BouncedGenerator::new(1, min, max, 3, duration);

        assert_eq!(gen.generate(&duration), 1);
        assert_eq!(gen.generate(&duration), 2);
        let param = gen.generate(&duration);
        assert!(param >= min && param <= max);
        assert!(param < 3 || param > 3);
        assert_eq!(gen.generate(&duration), 4);
        assert_eq!(gen.generate(&duration), 5);
        let param = gen.generate(&duration);
        assert!(param >= min && param <= max);
        assert!(param < 6 || param > 6);
    }

    #[test]
    fn should_not_bounce_when_range_is_less_than_4() {
        let duration = Duration::from_secs(1);
        let mut gen = BouncedGenerator::new(1, 0, 2, 3, duration);

        assert_eq!(gen.generate(&duration), 1);
        assert_eq!(gen.generate(&duration), 2);
        assert_eq!(gen.generate(&duration), 2);
        assert_eq!(gen.generate(&duration), 1);
        assert_eq!(gen.generate(&duration), 0);
    }

    #[test]
    fn should_reset_parameter() {
        let duration = Duration::from_secs(1);
        let mut generator = SequentialGenerator::new(1, 0, 100, duration);

        assert_eq!(generator.generate(&duration), 1);
        assert_eq!(generator.generate(&duration), 2);
        generator.reset(7);
        assert_eq!(generator.generate(&duration), 8);
        assert_eq!(generator.generate(&duration), 9);
    }
}
