use std::time;

use super::generic::Interpolator;

pub struct TransparentInterpolator<T> {
    value: T,
}

impl<T> TransparentInterpolator<T> {
    pub fn new(default: T) -> Self {
        Self { value: default }
    }
}

impl<T: Copy> Interpolator<T> for TransparentInterpolator<T> {
    fn interpolate(&mut self, _: &time::Duration) -> T {
        self.value
    }

    fn update(&mut self, value: T) {
        self.value = value;
    }

    fn reset(&mut self, value: T) {
        self.value = value;
    }
}
