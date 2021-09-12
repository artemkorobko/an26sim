use std::time::Duration;

pub trait Generator<T> {
    fn generate(&mut self, delta: Duration) -> T;
}
