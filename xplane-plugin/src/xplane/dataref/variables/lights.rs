use xplm::data::borrowed::{DataRef, FindError};
use xplm::data::{DataRead, DataReadWrite, ReadWrite};

const ON: i32 = 1;
const OFF: i32 = 0;

pub struct LightsDataRef {
    // Beacon lights switch [0,1]
    beacon_on: DataRef<i32, ReadWrite>,
    // Landing lights switch [0,1]
    landing_lights_on: DataRef<i32, ReadWrite>,
    // Navigation lights switch [0,1]
    navigation_lights_on: DataRef<i32, ReadWrite>,
}

impl LightsDataRef {
    pub fn new() -> Result<Self, FindError> {
        Ok(Self {
            beacon_on: DataRef::find("sim/cockpit2/switches/beacon_on")?.writeable()?,
            landing_lights_on: DataRef::find("sim/cockpit2/switches/landing_lights_on")?
                .writeable()?,
            navigation_lights_on: DataRef::find("sim/cockpit2/switches/navigation_lights_on")?
                .writeable()?,
        })
    }

    pub fn landing(&self) -> bool {
        self.landing_lights_on.get() == ON
    }

    pub fn set_landing(&mut self, state: bool) {
        match state {
            true => self.landing_lights_on.set(ON),
            false => self.landing_lights_on.set(OFF),
        }
    }

    pub fn navigation(&self) -> bool {
        self.navigation_lights_on.get() == ON
    }

    pub fn set_navigation(&mut self, state: bool) {
        match state {
            true => self.navigation_lights_on.set(ON),
            false => self.navigation_lights_on.set(OFF),
        }
    }

    pub fn beacon(&self) -> bool {
        self.beacon_on.get() == ON
    }

    pub fn set_beacon(&mut self, state: bool) {
        match state {
            true => self.beacon_on.set(ON),
            false => self.beacon_on.set(OFF),
        }
    }
}
