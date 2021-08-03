use std::time::Duration;

use crate::common::chain::Supplier;

#[derive(Default)]
pub struct DeltaTimeSupplier {
    delta: Duration,
}

impl DeltaTimeSupplier {
    pub fn update(&mut self, delta: Duration) {
        self.delta = delta;
    }
}

impl Supplier<Duration> for DeltaTimeSupplier {
    fn supply(&mut self) -> Duration {
        self.delta
    }
}
