use std::time::Duration;

pub enum IOState {
    Connected,
    Disconnected,
}

impl Default for IOState {
    fn default() -> Self {
        Self::Disconnected
    }
}

#[derive(Default)]
pub struct IOMetrics {
    pub state: IOState,
    pub transferred: usize,
    pub packets: usize,
    pub errors: usize,
    last_transferred: usize,
}

impl IOMetrics {
    pub fn bps(&mut self, delta: &Duration) -> f32 {
        let actual_transferred = self.transferred - self.last_transferred;
        let bps = actual_transferred as f32 / delta.as_secs_f32();
        self.last_transferred = self.transferred;
        bps
    }
}

#[cfg(test)]
mod test {
    use float_eq::assert_float_eq;

    use super::*;

    const PRECISION: f32 = 0.001;

    #[test]
    fn should_calc_bps() {
        let mut metrics = IOMetrics::default();
        metrics.transferred = 10;

        let bps1 = metrics.bps(&Duration::from_millis(500));
        let bps2 = metrics.bps(&Duration::from_millis(500));

        assert_float_eq!(bps1, 20.00, abs <= PRECISION);
        assert_float_eq!(bps2, 0.0, abs <= PRECISION);
    }
}
