use std::cell::Ref;

use super::dataref::collection::DataRefs;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct XPlaneInputParams {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub ailerons: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
    pub engine_left: f32,
    pub engine_right: f32,
    pub gear_front: f32,
    pub gear_left: f32,
    pub gear_right: f32,
    pub light_landing: bool,
    pub light_navigation: bool,
    pub light_beacon: bool,
    pub reset: bool,
}

impl From<Ref<'_, DataRefs>> for XPlaneInputParams {
    fn from(datarefs: Ref<DataRefs>) -> Self {
        let coords = datarefs.location.coords();
        let engines = datarefs.engines.get();
        let gears = datarefs.gears.get();
        Self {
            latitude: coords.latitude,
            longitude: coords.longitude,
            altitude: coords.altitude,
            heading: datarefs.orientation.heading(),
            pitch: datarefs.orientation.pitch(),
            roll: datarefs.orientation.roll(),
            ailerons: datarefs.surfaces.ailerons(),
            elevator: datarefs.surfaces.elevator(),
            rudder: datarefs.surfaces.rudder(),
            flaps: datarefs.general.fps(),
            engine_left: engines.left,
            engine_right: engines.right,
            gear_front: gears.front,
            gear_left: gears.left,
            gear_right: gears.right,
            light_landing: datarefs.lights.landing(),
            light_navigation: datarefs.lights.navigation(),
            light_beacon: datarefs.lights.beacon(),
            reset: false,
        }
    }
}
