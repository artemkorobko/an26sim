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
