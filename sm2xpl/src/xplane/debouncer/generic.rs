use std::time::Duration;

pub trait Debouncer<T> {
    fn debounce(&mut self, target: T, delta: &Duration) -> T;
    fn integrate(&mut self, delta: &Duration) -> T;
    fn assign(&mut self, target: T) -> T;
}
