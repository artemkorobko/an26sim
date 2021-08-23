use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadWrite};
use xplm_sys::{XPLMLocalToWorld, XPLMWorldToLocal};

use crate::xplane::input_params::Location;

pub struct Local {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct LocationDataRef {
    // Location of the plane in OpenGL coordinates (meters)
    local_x: DataRef<f64, ReadWrite>,
    // Location of the plane in OpenGL coordinates (meters)
    local_y: DataRef<f64, ReadWrite>,
    // Location of the plane in OpenGL coordinates (meters, up in the air)
    local_z: DataRef<f64, ReadWrite>,
}

impl LocationDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            local_x: DataRef::find("sim/flightmodel/position/local_x")?.writeable()?,
            local_y: DataRef::find("sim/flightmodel/position/local_y")?.writeable()?,
            local_z: DataRef::find("sim/flightmodel/position/local_z")?.writeable()?,
        })
    }

    pub fn get(&self) -> Location {
        let mut latitude: f64 = 0.0;
        let mut longitude: f64 = 0.0;
        let mut altitude: f64 = 0.0;
        unsafe {
            XPLMLocalToWorld(
                self.local_x.get(),
                self.local_y.get(),
                self.local_z.get(),
                &mut latitude,
                &mut longitude,
                &mut altitude,
            )
        };
        Location {
            latitude,
            longitude,
            altitude,
        }
    }

    pub fn set(&mut self, location: &Location) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            XPLMWorldToLocal(
                location.latitude,
                location.longitude,
                location.altitude,
                &mut x,
                &mut y,
                &mut z,
            );
        };
        self.local_x.set(x);
        self.local_y.set(y);
        self.local_z.set(z);
    }

    pub fn local(&self) -> Local {
        Local {
            x: self.local_x.get(),
            y: self.local_y.get(),
            z: self.local_z.get(),
        }
    }
}
