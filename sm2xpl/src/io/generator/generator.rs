use std::time::Duration;

pub trait Generator {
    fn generate(&mut self, delta: Duration) -> Vec<u8>;
    fn size_bytes(&self) -> usize;
}
