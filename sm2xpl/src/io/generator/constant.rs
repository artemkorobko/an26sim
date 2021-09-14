use std::time::Duration;

use super::{generator::Generator, parameter::Parameter};

pub struct ConstGenerator<T: Parameter> {
    value: T,
}

impl<T: Parameter> From<T> for ConstGenerator<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl<T: Parameter> Generator for ConstGenerator<T> {
    fn generate(&mut self, _: Duration) -> Vec<u8> {
        self.value.to_be_bytes_vec()
    }

    fn size_bytes(&self) -> usize {
        self.value.size_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_same_value() {
        let default = 100i16;
        let mut generator = ConstGenerator::from(default);

        let bytes = generator.generate(Duration::ZERO);
        assert_eq!(bytes, default.to_be_bytes());

        let bytes = generator.generate(Duration::from_secs(1));
        assert_eq!(bytes, default.to_be_bytes());
    }
}
