use std::time::Duration;

use super::generic::Generator;

pub struct ConstantGenerator {
    value: i16,
}

impl ConstantGenerator {
    pub fn new(default: i16) -> Self {
        Self { value: default }
    }
}

impl Generator for ConstantGenerator {
    fn generate(&mut self, _: Duration) -> i16 {
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
