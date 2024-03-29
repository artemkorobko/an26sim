use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, ReadWrite};

pub struct ViewDataRef {
    acf_pex: DataRef<f32, ReadWrite>,
    acf_pey: DataRef<f32, ReadWrite>,
    acf_pez: DataRef<f32, ReadWrite>,
}

impl ViewDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            // Position of pilot's head relative to CG, X
            acf_pex: DataRef::find("sim/aircraft/view/acf_peX")?.writeable()?,
            // Position of pilot's head relative to CG, Y
            acf_pey: DataRef::find("sim/aircraft/view/acf_peY")?.writeable()?,
            // Position of pilot's head relative to CG, Z
            acf_pez: DataRef::find("sim/aircraft/view/acf_peZ")?.writeable()?,
        })
    }

    pub fn x(&self) -> f32 {
        self.acf_pex.get()
    }

    pub fn y(&self) -> f32 {
        self.acf_pey.get()
    }

    pub fn z(&self) -> f32 {
        self.acf_pez.get()
    }
}
