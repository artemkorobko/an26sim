use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, ArrayReadWrite, ReadWrite};

use crate::xplane::params::Engines;

const LEFT_IDX: usize = 0;
const RIGHT_IDX: usize = 1;
const DEFAULT: f32 = 0.0;

pub struct EnginesDataRef {
    // Prop speed in radians/second [0.0..100.0]
    point_tacrad: DataRef<[f32], ReadWrite>,
}

impl EnginesDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            point_tacrad: DataRef::find("sim/flightmodel/engine/POINT_tacrad")?.writeable()?,
        })
    }

    pub fn get(&self) -> Engines {
        let values = self.point_tacrad.as_vec();
        Engines {
            left: *values.get(LEFT_IDX).unwrap_or(&DEFAULT),
            right: *values.get(RIGHT_IDX).unwrap_or(&DEFAULT),
        }
    }

    pub fn set(&mut self, engines: &Engines) {
        let mut values = self.point_tacrad.as_vec();
        values.get_mut(LEFT_IDX).map(|value| *value = engines.left);
        values
            .get_mut(RIGHT_IDX)
            .map(|value| *value = engines.right);
        self.point_tacrad.set(&values);
    }
}
