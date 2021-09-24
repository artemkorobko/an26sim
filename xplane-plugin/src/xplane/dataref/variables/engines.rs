use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, ArrayReadWrite, ReadWrite};

const LEFT_IDX: usize = 0;
const RIGHT_IDX: usize = 1;
const DEFAULT: f32 = 0.0;

pub struct EnginesState {
    pub left: f32,
    pub right: f32,
}

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

    pub fn get(&self) -> EnginesState {
        let values = self.point_tacrad.as_vec();
        let left = *values.get(LEFT_IDX).unwrap_or(&DEFAULT);
        let right = *values.get(RIGHT_IDX).unwrap_or(&DEFAULT);
        EnginesState { left, right }
    }

    pub fn set(&mut self, left: f32, right: f32) {
        let mut values = self.point_tacrad.as_vec();
        values.get_mut(LEFT_IDX).map(|value| *value = left);
        values.get_mut(RIGHT_IDX).map(|value| *value = right);
        self.point_tacrad.set(&values);
    }
}
