use std::time::Duration;

use num_traits::PrimInt;

pub trait Generator<T: PrimInt> {
    fn generate(&mut self, delta: &Duration) -> T;
    fn reset(&mut self, param: T);
}
