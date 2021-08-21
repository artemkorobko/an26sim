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

mod test {
    use super::*;

    #[test]
    fn should_supply_delta_time() {
        let mut supplier = DeltaTimeSupplier::default();
        assert_eq!(supplier.supply(), Duration::ZERO);
        let duration = Duration::from_secs(1);
        supplier.update(duration);
        assert_eq!(supplier.supply(), duration);
    }
}
