use std::time::Duration;

use crate::shared::pipeline::Supplier;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_updated_delta_time() {
        let duration = Duration::from_secs(1);
        let mut supplier = DeltaTimeSupplier::default();
        assert_eq!(supplier.supply(), Duration::ZERO);

        supplier.update(duration);

        assert_eq!(supplier.supply(), duration);
    }
}
