use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadOnly, ReadWrite};
use xplm_sys::{XPLMLocalToWorld, XPLMWorldToLocal};

#[derive(Default)]
pub struct Coords {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

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
    // AGL meters above the ground level
    y_agl: DataRef<f32, ReadOnly>,
}

impl LocationDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            local_x: DataRef::find("sim/flightmodel/position/local_x")?.writeable()?,
            local_y: DataRef::find("sim/flightmodel/position/local_y")?.writeable()?,
            local_z: DataRef::find("sim/flightmodel/position/local_z")?.writeable()?,
            y_agl: DataRef::find("sim/flightmodel/position/y_agl")?,
        })
    }

    pub fn agl(&self) -> f32 {
        self.y_agl.get()
    }

    pub fn coords(&self) -> Coords {
        let mut coords = Coords::default();
        unsafe {
            XPLMLocalToWorld(
                self.local_x.get(),
                self.local_y.get(),
                self.local_z.get(),
                &mut coords.latitude,
                &mut coords.longitude,
                &mut coords.altitude,
            )
        };
        coords
    }

    pub fn set_coords(&mut self, latitude: f64, longitude: f64, altitude: f64) {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        unsafe {
            XPLMWorldToLocal(latitude, longitude, altitude, &mut x, &mut y, &mut z);
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
