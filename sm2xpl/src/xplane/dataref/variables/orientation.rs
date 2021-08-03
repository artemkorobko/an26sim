use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadWrite};

use crate::xplane::params::Orientation;

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

    pub fn get(&self) -> Orientation {
        Orientation {
            heading: self.psi.get(),
            pitch: self.theha.get(),
            roll: self.phi.get(),
        }
    }

    pub fn set(&mut self, orientation: &Orientation) {
        self.psi.set(orientation.heading);
        self.theha.set(orientation.pitch);
        self.phi.set(orientation.roll);
    }
}
