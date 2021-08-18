use std::{cell::RefCell, rc::Rc, time::Duration};

use xplm::debugln;

use crate::{
    common::{
        chain::{Consumer, Supplier},
        timer::TimeCounter,
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
    inspector: Rc<RefCell<InspectorWindow>>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: TimeCounter,
}

impl XPlaneInspectorUpdater {
    pub fn new(
        inspector: Rc<RefCell<InspectorWindow>>,
        delta: Rc<RefCell<DeltaTimeSupplier>>,
    ) -> Self {
        Self {
            inspector,
            delta,
            timer: TimeCounter::new(Duration::from_millis(50)),
        }
    }

    fn should_update_inspector(&mut self) -> bool {
        if self.inspector.borrow().visible() {
            let delta = self.delta.borrow_mut().supply();
            self.timer.count(&delta).is_some()
        } else {
            false
        }
    }

    fn update_inspector(&mut self, input: &XPlaneOutputParams) {
        let result = self.inspector.borrow_mut().update(input);
        if let Err(error) = result {
            debugln!("{}", error.to_string());
        }
    }
}

impl Consumer<XPlaneOutputParams> for XPlaneInspectorUpdater {
    fn consume(&mut self, input: &XPlaneOutputParams) {
        if self.should_update_inspector() {
            self.update_inspector(input);
        }
    }
}