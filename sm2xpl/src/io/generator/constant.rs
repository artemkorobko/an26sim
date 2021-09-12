use std::time::Duration;

use super::generic::Generator;

pub struct ConstantGenerator<T> {
    value: T,
}

impl<T> ConstantGenerator<T> {
    pub fn new(default: T) -> Self {
        Self { value: default }
    }
}

impl<T: Copy> Generator<T> for ConstantGenerator<T> {
    fn generate(&mut self, _: Duration) -> T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_constant() {
        let default = 100;

        let mut generator = ConstantGenerator::new(default);

        assert_eq!(generator.generate(Duration::ZERO), default);
        assert_eq!(generator.generate(Duration::ZERO), default);
    }
}
