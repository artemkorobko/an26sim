use std::{cell::RefCell, rc::Rc};

use crate::{shared::pipeline::Consumer, xplane::input_params::XPlaneInputParams};

use super::collection::DataRefs;

pub struct XPlaneDataRefUpdater {
    datarefs: Rc<RefCell<DataRefs>>,
}

impl XPlaneDataRefUpdater {
    pub fn new(datarefs: Rc<RefCell<DataRefs>>) -> Self {
        Self { datarefs }
    }

    fn update_datarefs(&mut self, input: &XPlaneInputParams) {
        let mut datarefs = self.datarefs.borrow_mut();
        datarefs
            .location
            .set_coords(input.latitude, input.longitude, input.altitude);
        datarefs.orientation.set_heading(input.heading);
        datarefs.orientation.set_pitch(input.pitch);
        datarefs.orientation.set_roll(input.roll);
        datarefs.surfaces.set_ailerons(input.ailerons);
        datarefs.surfaces.set_elevator(input.elevator);
        datarefs.surfaces.set_rudder(input.rudder);
        datarefs.surfaces.set_flaps(input.flaps);
        datarefs.engines.set(input.engine_left, input.engine_right);
        datarefs
            .gears
            .set(input.gear_front, input.gear_left, input.gear_right);
        datarefs.lights.set_landing(input.light_landing);
        datarefs.lights.set_navigation(input.light_navigation);
        datarefs.lights.set_beacon(input.light_beacon);
    }

    fn should_update_datarefs(&self) -> bool {
        self.datarefs.borrow().general.is_physics_disabled()
    }
}

impl Consumer<XPlaneInputParams> for XPlaneDataRefUpdater {
    fn consume(&mut self, input: &XPlaneInputParams) {
        if self.should_update_datarefs() {
            self.update_datarefs(input);
        }
    }
}
