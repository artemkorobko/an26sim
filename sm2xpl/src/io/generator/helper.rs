use super::{
    bounced::BouncedGenerator, constant::ConstGenerator, parameter::Parameter,
    sequential::SequentialGenerator,
};

pub trait ToGenerator<T: Parameter> {
    fn to_const_generator(self) -> ConstGenerator<T>;
    fn to_sequential_generator(self) -> SequentialGenerator<T>;
}

macro_rules! impl_to_generator {
    ($type:ty) => {
        impl ToGenerator<$type> for $type {
            fn to_const_generator(self) -> ConstGenerator<$type> {
                ConstGenerator::from(self)
            }

            fn to_sequential_generator(self) -> SequentialGenerator<$type> {
                SequentialGenerator::from(self)
            }
        }
    };
}

impl_to_generator!(u16);
impl_to_generator!(i16);
impl_to_generator!(u32);

pub trait ToBounced<T: Parameter> {
    fn to_bounced_generator(self) -> BouncedGenerator<T>;
}

macro_rules! impl_to_bounced {
    ($generator:ident) => {
        impl<T: 'static + Parameter + Copy> ToBounced<T> for $generator<T> {
            fn to_bounced_generator(self) -> BouncedGenerator<T> {
                BouncedGenerator::bounce(Box::new(self))
            }
        }
    };
}

impl_to_bounced!(ConstGenerator);
impl_to_bounced!(SequentialGenerator);

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bytes::BufMut;

    use crate::io::generator::generator::Generator;

    use super::*;

    #[test]
    fn u16_to_const_generator() {
        let default = 10u16;
        let mut gen = default.to_const_generator();
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), default.to_be_bytes());
    }

    #[test]
    fn u16_to_sequential_generator() {
        let default = 20u16;
        let step = 5;
        let mut gen = default.to_sequential_generator().with_step(step);
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), (default + step).to_be_bytes());
    }

    #[test]
    fn i16_to_const_generator() {
        let default = 1i16;
        let mut gen = default.to_const_generator();
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), default.to_be_bytes());
    }

    #[test]
    fn i16_to_sequential_generator() {
        let default = -5i16;
        let step = 15;
        let mut gen = default.to_sequential_generator().with_step(step);
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), (default + step).to_be_bytes());
    }

    #[test]
    fn u32_to_const_generator() {
        let default = 123u32;
        let mut gen = default.to_const_generator();
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 4);
        assert_eq!(buf.into_inner(), default.to_be_bytes());
    }

    #[test]
    fn u32_to_sequential_generator() {
        let default = 55u32;
        let step = 8;
        let mut gen = default.to_sequential_generator().with_step(step);
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 4);
        assert_eq!(buf.into_inner(), (default + step).to_be_bytes());
    }

    #[test]
    fn const_to_bounced_generator() {
        let default = 73i16;
        let mut gen = default.to_const_generator().to_bounced_generator();
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), default.to_be_bytes());
    }

    #[test]
    fn sequential_to_bounced_generator() {
        let default = 73i16;
        let step = 22;
        let mut gen = default
            .to_sequential_generator()
            .with_step(step)
            .to_bounced_generator();
        let mut buf = Vec::<u8>::new().writer();

        let size = gen.write(Duration::ZERO, &mut buf).unwrap();

        assert_eq!(size, 2);
        assert_eq!(buf.into_inner(), (default + step).to_be_bytes());
    }
}
