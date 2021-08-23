use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadWrite};

use crate::xplane::input_params::Surfaces;

pub struct SurfacesDataRef {
    yoke_roll_ratio: DataRef<f32, ReadWrite>,
    yoke_pitch_ratio: DataRef<f32, ReadWrite>,
    yoke_heading_ratio: DataRef<f32, ReadWrite>,
    flaprat: DataRef<f32, ReadWrite>,
}

impl SurfacesDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            // Deflection of the joystick axis controlling roll [turn left -1.0..1.0 turn right]
            yoke_roll_ratio: DataRef::find("sim/joystick/yoke_roll_ratio")?.writeable()?,
            // Deflection of the joystick axis controlling pitch [decend -1.0..1.0 climb]
            yoke_pitch_ratio: DataRef::find("sim/joystick/yoke_pitch_ratio")?.writeable()?,
            // Deflection of the joystick axis controlling yaw [turn left -1.0..1.0 turn right]
            yoke_heading_ratio: DataRef::find("sim/joystick/yoke_heading_ratio")?.writeable()?,
            // Actual flap deployment [0.0..1.0]
            flaprat: DataRef::find("sim/flightmodel/controls/flaprat")?.writeable()?,
        })
    }

    pub fn get(&self) -> Surfaces {
        Surfaces {
            ailerons: self.yoke_roll_ratio.get(),
            elevator: self.yoke_pitch_ratio.get(),
            rudder: self.yoke_heading_ratio.get(),
            flaps: self.flaprat.get(),
        }
    }

    pub fn set(&mut self, surfaces: &Surfaces) {
        self.yoke_roll_ratio.set(surfaces.ailerons);
        self.yoke_pitch_ratio.set(surfaces.elevator);
        self.yoke_heading_ratio.set(surfaces.rudder);
        self.flaprat.set(surfaces.flaps);
    }
}
