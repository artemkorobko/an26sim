use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{ArrayRead, ArrayReadWrite, DataRead, DataReadWrite, ReadOnly, ReadWrite};

const ENABLED: i32 = 0;
const DISABLED: i32 = 1;

pub struct GeneralDataRef {
    // The frame rate period.
    // Use the reciprocal to get the frame rate (e.g. 1/mnw.prd).
    // Writable via override_timestep in 11.10
    frame_rate_period: DataRef<f32, ReadOnly>,
    // Override position updates of this plane [0,1]
    override_plane_path: DataRef<[i32], ReadWrite>,
    // Override control of the joystick deflections
    // (overrides stick, yoke, pedals, keys, mouse, and auto-coordination) [0,1]
    override_joystick: DataRef<i32, ReadWrite>,
}

impl GeneralDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            frame_rate_period: DataRef::find("sim/operation/misc/frame_rate_period")?,
            override_plane_path: DataRef::find("sim/operation/override/override_planepath")?
                .writeable()?,
            override_joystick: DataRef::find("sim/operation/override/override_joystick")?
                .writeable()?,
        })
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.frame_rate_period.get()
    }

    pub fn is_physics_enabled(&self) -> bool {
        self.override_plane_path
            .as_vec()
            .get(0)
            .map(|value| *value == ENABLED && self.override_joystick.get() == ENABLED)
            .unwrap_or(false)
    }

    pub fn enable_physics(&mut self) {
        self.set_physics_state(ENABLED);
    }

    pub fn disable_physics(&mut self) {
        self.set_physics_state(DISABLED);
    }

    pub fn is_physics_disabled(&self) -> bool {
        !self.is_physics_enabled()
    }

    fn set_physics_state(&mut self, value: i32) {
        let mut flags = self.override_plane_path.as_vec();
        if let Some(override_plane_path) = flags.get_mut(0) {
            *override_plane_path = value;
            self.override_plane_path.set(&flags);
            self.override_joystick.set(value);
        }
    }
}
