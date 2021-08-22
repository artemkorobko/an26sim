use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use xplm::flight_loop::{FlightLoopCallback, LoopState};

use crate::{
    common::chain::{Chain, Mapper},
    io::{delta::DeltaTimeSupplier, generator::usb::USBParamGenerator},
    plugin_event::PluginEvent,
    xplane::{
        consumer::{XPlaneDataRefUpdater, XPlaneInspectorUpdater},
        dataref::collection::DataRefs,
        debouncer::XPlaneParamDebouncer,
        inspector::window::InspectorWindow,
        interpolator::XPlaneParamInterpolator,
        mapper::{
            input::{SM2MXPlaneInputMapper, XPlaneSM2MInputMapper},
            output::XPlaneSM2MOutputMapper,
        },
        menu::{instance::PluginMenu, item::MenuItem},
        params::XPlaneInputParams,
        supplier::XPlaneOutputSupplier,
    },
};

type InputChain = Chain<XPlaneInputParams>;
type OutputChain = Chain<Vec<u16>>;

pub struct Controller {
    menu: Box<PluginMenu>,
    datarefs: Rc<RefCell<DataRefs>>,
    inspector: Rc<RefCell<InspectorWindow>>,
    rx: Receiver<PluginEvent>,
    delta_supplier: Rc<RefCell<DeltaTimeSupplier>>,
    input_chain: Option<InputChain>,
    output_chain: OutputChain,
}

impl Controller {
    pub fn new(
        menu: Box<PluginMenu>,
        inspector: InspectorWindow,
        datarefs: DataRefs,
        rx: Receiver<PluginEvent>,
    ) -> Self {
        let datarefs = Rc::new(RefCell::new(datarefs));
        let inspector = Rc::new(RefCell::new(inspector));
        let delta = Rc::new(RefCell::new(DeltaTimeSupplier::default()));
        let ochain = build_default_output_chain(datarefs.clone());

        Self {
            menu,
            datarefs,
            inspector,
            rx,
            delta_supplier: delta,
            input_chain: None,
            output_chain: ochain,
        }
    }

    fn handle_events(&mut self) {
        if let Ok(event) = self.rx.try_recv() {
            match event {
                PluginEvent::EnablePhysics => self.datarefs.borrow_mut().general.enable_physics(),
                PluginEvent::DisablePhysics => self.datarefs.borrow_mut().general.disable_physics(),
                PluginEvent::ShowDebugWindow => self.inspector.borrow_mut().show(),
                PluginEvent::HideDebugWindow => self.inspector.borrow_mut().hide(),
                PluginEvent::StartTest => self.start_test(),
                PluginEvent::StopTest => self.stop_test(),
            }
        }
    }

    fn start_test(&mut self) {
        self.input_chain = Some(self.build_generator_input_chain());
        self.menu.uncheck_item(MenuItem::Physics);
        self.menu.check_item(MenuItem::Inspector);
        self.datarefs.borrow_mut().general.disable_physics();
        self.inspector.borrow_mut().show();
    }

    fn stop_test(&mut self) {
        self.input_chain = None;
    }

    fn build_generator_input_chain(&self) -> InputChain {
        let input_params = self.datarefs.borrow().as_input();
        let params = XPlaneSM2MInputMapper::default().map(input_params);
        let mut generator = USBParamGenerator::dynamic(self.delta_supplier.clone());
        generator.update_params(&params);
        Chain::supply(generator)
            .map(SM2MXPlaneInputMapper::default())
            .map(XPlaneParamDebouncer::new())
            .map(XPlaneParamInterpolator::new(
                input_params,
                self.delta_supplier.clone(),
            ))
            .consume(XPlaneDataRefUpdater::new(self.datarefs.clone()))
            .consume(XPlaneInspectorUpdater::new(
                self.datarefs.clone(),
                self.inspector.clone(),
                self.delta_supplier.clone(),
            ))
    }

    fn execute(&mut self, delta: Duration) {
        if let Some(input) = &mut self.input_chain {
            self.delta_supplier.borrow_mut().update(delta);
            input.execute();
            self.output_chain.execute();
        }
    }
}

fn build_default_output_chain(datarefs: Rc<RefCell<DataRefs>>) -> OutputChain {
    Chain::supply(XPlaneOutputSupplier::new(datarefs.clone())).map(XPlaneSM2MOutputMapper)
}

impl FlightLoopCallback for Controller {
    fn flight_loop(&mut self, state: &mut LoopState) {
        self.handle_events();
        self.execute(state.since_last_call())
    }
}
