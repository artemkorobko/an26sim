use std::{cell::RefCell, rc::Rc, time::Duration};

use xplm::debugln;

use crate::{
    common::{
        chain::{Consumer, Supplier},
        timer::DeltaCounter,
    },
    io::delta::DeltaTimeSupplier,
};

use super::{
    dataref::collection::DataRefs,
    inspector::window::InspectorWindow,
    params::{XPlaneInputParams, XPlaneOutputParams},
};

pub struct XPlaneDataRefUpdater {
    datarefs: Rc<RefCell<DataRefs>>,
}

impl XPlaneDataRefUpdater {
    pub fn new(datarefs: Rc<RefCell<DataRefs>>) -> Self {
        Self { datarefs }
    }

    fn update_datarefs(&mut self, input: &XPlaneInputParams) {
        self.datarefs.borrow_mut().set(input);
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

pub struct XPlaneInspectorUpdater {
    datarefs: Rc<RefCell<DataRefs>>,
    inspector: Rc<RefCell<InspectorWindow>>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: DeltaCounter,
}

impl XPlaneInspectorUpdater {
    pub fn new(
        datarefs: Rc<RefCell<DataRefs>>,
        inspector: Rc<RefCell<InspectorWindow>>,
        delta: Rc<RefCell<DeltaTimeSupplier>>,
    ) -> Self {
        Self {
            datarefs,
            inspector,
            delta,
            timer: DeltaCounter::immediate(Duration::from_millis(50)),
        }
    }

    fn should_update_inspector(&mut self) -> bool {
        if self.inspector.borrow().visible() {
            let delta = self.delta.borrow_mut().supply();
            self.timer.count(&delta).is_elapsed()
        } else {
            false
        }
    }

    fn update_inspector(&mut self, input: &XPlaneInputParams) {
        let datarefs = self.datarefs.borrow();
        let local = datarefs.location.local();
        let result = self.inspector.borrow_mut().update(
            &datarefs.as_input(),
            &datarefs.general.get(),
            &datarefs.view.get(),
            datarefs.terrain_probe.distance(local.x, local.y, local.z),
        );
        if let Err(error) = result {
            debugln!("{}", error.to_string());
        }
    }
}

impl Consumer<XPlaneInputParams> for XPlaneInspectorUpdater {
    fn consume(&mut self, input: &XPlaneInputParams) {
        if self.should_update_inspector() {
            self.update_inspector(input);
        }
    }
}
