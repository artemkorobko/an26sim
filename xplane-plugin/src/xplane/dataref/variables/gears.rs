use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, ArrayReadWrite, ReadWrite};

const FRONT_IDX: usize = 0;
const LEFT_IDX: usize = 1;
const RIGHT_IDX: usize = 2;
const DEFAULT: f32 = 0.0;

pub struct GearsState {
    pub front: f32,
    pub left: f32,
    pub right: f32,
}

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

    pub fn get(&self) -> GearsState {
        let values = self.acf_gear_deploy.as_vec();
        let front = *values.get(FRONT_IDX).unwrap_or(&DEFAULT);
        let left = *values.get(LEFT_IDX).unwrap_or(&DEFAULT);
        let right = *values.get(RIGHT_IDX).unwrap_or(&DEFAULT);
        GearsState { front, left, right }
    }

    pub fn set(&mut self, front: f32, left: f32, right: f32) {
        let mut values = self.acf_gear_deploy.as_vec();
        values.get_mut(FRONT_IDX).map(|value| *value = front);
        values.get_mut(LEFT_IDX).map(|value| *value = left);
        values.get_mut(RIGHT_IDX).map(|value| *value = right);
        self.acf_gear_deploy.set(&values);
    }
}
