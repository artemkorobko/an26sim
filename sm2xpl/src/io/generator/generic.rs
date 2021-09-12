use std::time::Duration;

pub trait Generator {
    fn generate(&mut self, delta: Duration) -> i16;
}
