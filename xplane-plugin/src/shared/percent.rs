use std::ops::{Mul, Sub};

pub trait Percent: Sized + Copy + Mul<Output = Self> + Sub<Output = Self> {
    fn scale(&self, src_min: Self, src_max: Self, dst_min: Self, dst_max: Self) -> Self;
    fn value_from_percent(&self, min: Self, max: Self) -> Self;
    fn percent_from_value(&self, min: Self, max: Self) -> Self;
}

macro_rules! impl_percent_for {
    ($type:ty) => {
        impl Percent for $type {
            fn value_from_percent(&self, min: Self, max: Self) -> Self {
                min + ((max - min) * 0.01) * self
            }

            fn percent_from_value(&self, min: Self, max: Self) -> Self {
                ((self - min) * 100.0) / (max - min)
            }

            fn scale(&self, src_min: Self, src_max: Self, dst_min: Self, dst_max: Self) -> Self {
                let percent = self.percent_from_value(src_min, src_max);
                percent.value_from_percent(dst_min, dst_max)
            }
        }
    };
}

impl_percent_for!(f32);
impl_percent_for!(f64);

#[cfg(test)]
mod test {
    use float_eq::assert_float_eq;

    use super::*;

    const F32_PRECISION: f32 = 0.001;
    const F64_PRECISION: f64 = F32_PRECISION as f64;

    #[test]
    fn get_f32_percent_from_value() {
        let percent = Percent::percent_from_value(&0.0f32, -50.0f32, 50.0f32);
        assert_float_eq!(percent, 50.00, abs <= F32_PRECISION);
    }

    #[test]
    fn get_f64_percent_from_value() {
        let percent = Percent::percent_from_value(&0.0f64, -50.0f64, 50.0f64);
        assert_float_eq!(percent, 50.00, abs <= F64_PRECISION);
    }

    #[test]
    fn get_f32_value_from_percent() {
        let value = Percent::value_from_percent(&50.0f32, -50.0f32, 50.0f32);
        assert_float_eq!(value, 0.0, abs <= F32_PRECISION);
    }

    #[test]
    fn get_f64_value_from_percent() {
        let value = Percent::value_from_percent(&50.0f64, -50.0f64, 50.0f64);
        assert_float_eq!(value, 0.0, abs <= F64_PRECISION);
    }

    #[test]
    fn scale_f32() {
        let value = Percent::scale(&25.0f32, 0.0f32, 50.0f32, -100.0f32, -50.0f32);
        assert_float_eq!(value, -75.0, abs <= F32_PRECISION);
    }

    #[test]
    fn scale_f64() {
        let value = Percent::scale(&25.0f64, 0.0f64, 50.0f64, -100.0f64, -50.0f64);
        assert_float_eq!(value, -75.0, abs <= F64_PRECISION);
    }
}
