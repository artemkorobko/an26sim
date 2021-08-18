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

mod test {
    #[test]
    fn should_get_f32_percent_from_value() {
        let percent = super::Percent::percent_from_value(&0.0f32, -50.0f32, 50.0f32);
        assert!(percent > 49.99f32 && percent < 50.01f32);
    }

    #[test]
    fn should_get_f64_percent_from_value() {
        let percent = super::Percent::percent_from_value(&0.0f64, -50.0f64, 50.0f64);
        assert!(percent > 49.99f64 && percent < 50.01f64);
    }

    #[test]
    fn should_get_f32_value_from_percent() {
        let value = super::Percent::value_from_percent(&50.0f32, -50.0f32, 50.0f32);
        assert!(value > -0.01f32 && value < 0.01f32);
    }

    #[test]
    fn should_get_f64_value_from_percent() {
        let value = super::Percent::value_from_percent(&50.0f64, -50.0f64, 50.0f64);
        assert!(value > -0.01f64 && value < 0.01f64);
    }

    #[test]
    fn should_scale_f32() {
        let value = super::Percent::scale(&25.0f32, 0.0f32, 50.0f32, -100.0f32, -50.0f32);
        assert!(value > -75.01f32 && value < -74.99f32);
    }

    #[test]
    fn should_scale_f64() {
        let value = super::Percent::scale(&25.0f64, 0.0f64, 50.0f64, -100.0f64, -50.0f64);
        assert!(value > -75.01f64 && value < -74.99f64);
    }
}
