use std::{io, time::Duration};

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
    fn write(&mut self, _: Duration, buf: &mut dyn io::Write) -> io::Result<usize> {
        self.value.write(buf)
    }

    fn size_bytes(&self) -> usize {
        self.value.size_bytes()
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;

    use super::*;

    #[test]
    fn generate_same_value() {
        let default = 100i16;
        let mut generator = ConstGenerator::from(default);

        let mut buf = Vec::<u8>::new().writer();
        let size = generator.write(Duration::ZERO, &mut buf).unwrap();
        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), default.to_be_bytes());

        let mut buf = Vec::<u8>::new().writer();
        let size = generator.write(Duration::from_secs(1), &mut buf).unwrap();
        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), default.to_be_bytes());
    }
}
