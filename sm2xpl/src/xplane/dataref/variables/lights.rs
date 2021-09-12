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

    pub fn navigation(&self) -> bool {
        self.navigation_lights_on.get() == ON
    }

    pub fn beacon(&self) -> bool {
        self.beacon_on.get() == ON
    }

    pub fn landing_on(&mut self) {
        self.landing_lights_on.set(ON);
    }

    pub fn landing_off(&mut self) {
        self.landing_lights_on.set(OFF);
    }

    pub fn navigation_on(&mut self) {
        self.navigation_lights_on.set(ON);
    }

    pub fn navigation_off(&mut self) {
        self.navigation_lights_on.set(OFF);
    }

    pub fn beacon_on(&mut self) {
        self.beacon_on.set(ON);
    }

    pub fn beacon_off(&mut self) {
        self.beacon_on.set(OFF);
    }
}
