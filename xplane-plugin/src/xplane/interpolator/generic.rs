use std::time::Duration;

pub trait Interpolator<T> {
    fn interpolate(&mut self, delta: &Duration) -> T;
    fn update(&mut self, value: T);
    fn reset(&mut self, value: T);
}
