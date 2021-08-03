use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, ArrayReadWrite, ReadWrite};

use crate::xplane::params::Gears;

const FRONT_IDX: usize = 0;
const LEFT_IDX: usize = 1;
const RIGHT_IDX: usize = 2;
const DEFAULT: f32 = 0.0;

pub struct GearsDataRef {
    // Landing gear deployment [0.0..1.0]
    acf_gear_deploy: DataRef<[f32], ReadWrite>,
}

impl GearsDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            acf_gear_deploy: DataRef::find("sim/aircraft/parts/acf_gear_deploy")?.writeable()?,
        })
    }

    pub fn get(&self) -> Gears {
        let values = self.acf_gear_deploy.as_vec();
        Gears {
            front: *values.get(FRONT_IDX).unwrap_or(&DEFAULT),
            left: *values.get(LEFT_IDX).unwrap_or(&DEFAULT),
            right: *values.get(RIGHT_IDX).unwrap_or(&DEFAULT),
        }
    }

    pub fn set(&mut self, gears: &Gears) {
        let mut values = self.acf_gear_deploy.as_vec();
        values.get_mut(FRONT_IDX).map(|value| *value = gears.front);
        values.get_mut(LEFT_IDX).map(|value| *value = gears.left);
        values.get_mut(RIGHT_IDX).map(|value| *value = gears.right);
        self.acf_gear_deploy.set(&values);
    }
}
