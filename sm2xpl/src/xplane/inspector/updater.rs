use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{
    shared::{
        delta::DeltaTimeSupplier,
        pipeline::{Consumer, Supplier},
        timer::{DeltaCounter, Elapsed},
    },
    io::metrics::IOMetrics,
    xplane::{dataref::collection::DataRefs, input_params::XPlaneInputParams},
};

use super::window::InspectorWindow;

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
            match self.timer.count(delta) {
                Elapsed::Yes(diff) => (true, self.timer.delay() + diff),
                Elapsed::No => (false, Duration::ZERO),
            }
        } else {
            (false, Duration::ZERO)
        }
    }

    fn update_inspector(&mut self, _: &XPlaneInputParams, delta: &Duration) {
        let datarefs = self.datarefs.borrow();
        let result = self.inspector.borrow_mut().update(
            &datarefs,
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
