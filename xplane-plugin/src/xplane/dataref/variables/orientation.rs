use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadWrite};

pub struct OrientationDataRef {
    // True heading of the plane in degrees from the Z axis (degrees) [0.0..360.0]
    psi: DataRef<f32, ReadWrite>,
    // Pitch relative to the plane normal to the Y axis in degrees (degrees) [0.0..360.0]
    theha: DataRef<f32, ReadWrite>,
    // Roll of the plane (degrees) [0.0..360.0]
    phi: DataRef<f32, ReadWrite>,
}

impl OrientationDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            psi: DataRef::find("sim/flightmodel/position/psi")?.writeable()?,
            theha: DataRef::find("sim/flightmodel/position/theta")?.writeable()?,
            phi: DataRef::find("sim/flightmodel/position/phi")?.writeable()?,
        })
    }

    pub fn heading(&self) -> f32 {
        self.psi.get()
    }

    pub fn set_heading(&mut self, value: f32) {
        self.psi.set(value);
    }

    pub fn pitch(&self) -> f32 {
        self.theha.get()
    }

    pub fn set_pitch(&mut self, value: f32) {
        self.theha.set(value);
    }

    pub fn roll(&self) -> f32 {
        self.phi.get()
    }

    pub fn set_roll(&mut self, value: f32) {
        self.phi.set(value);
    }
}
