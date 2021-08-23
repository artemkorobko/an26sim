use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    common::{
        chain::{Consumer, Supplier},
        timer::DeltaCounter,
    },
    io::{delta::DeltaTimeSupplier, metrics::IOMetrics},
};

use super::{
    dataref::collection::DataRefs, input_params::XPlaneInputParams,
    inspector::window::InspectorWindow,
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
    input: Rc<RefCell<IOMetrics>>,
    output: Rc<RefCell<IOMetrics>>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
    timer: DeltaCounter,
}

impl XPlaneInspectorUpdater {
    pub fn new(
        datarefs: Rc<RefCell<DataRefs>>,
        inspector: Rc<RefCell<InspectorWindow>>,
        input: Rc<RefCell<IOMetrics>>,
        output: Rc<RefCell<IOMetrics>>,
        delta: Rc<RefCell<DeltaTimeSupplier>>,
    ) -> Self {
        Self {
            datarefs,
            inspector,
            input,
            output,
            delta,
            timer: DeltaCounter::immediate(Duration::from_millis(50)),
        }
    }

    fn should_update_inspector(&mut self) -> (bool, Duration) {
        if self.inspector.borrow().visible() {
            let delta = self.delta.borrow_mut().supply();
            match self.timer.count(&delta) {
                crate::common::timer::Elapsed::Yes(diff) => (true, self.timer.delay() + diff),
                crate::common::timer::Elapsed::No => (false, Duration::ZERO),
            }
        } else {
            (false, Duration::ZERO)
        }
    }

    fn update_inspector(&mut self, input: &XPlaneInputParams, delta: &Duration) {
        let datarefs = self.datarefs.borrow();
        let local = datarefs.location.local();
        let result = self.inspector.borrow_mut().update(
            &datarefs.as_input(),
            &datarefs.general.get(),
            &datarefs.view.get(),
            datarefs.terrain_probe.distance(local.x, local.y, local.z),
            &mut self.input.borrow_mut(),
            &mut self.output.borrow_mut(),
            delta,
        );
        if let Err(error) = result {
            xplm::debugln!("{}", error.to_string());
        }
    }
}

impl Consumer<XPlaneInputParams> for XPlaneInspectorUpdater {
    fn consume(&mut self, input: &XPlaneInputParams) {
        let (should_update, delta) = self.should_update_inspector();
        if should_update {
            self.update_inspector(input, &delta);
        }
    }
}
