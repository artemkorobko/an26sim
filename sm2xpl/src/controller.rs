use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use xplm::flight_loop::{FlightLoopCallback, LoopState};

use crate::{
    common::{
        chain::{Chain, Mapper},
        delta::DeltaTimeSupplier,
    },
    io::{generator::usb::USBParamGenerator, input_params::InputParamType, metrics::IOMetrics},
    plugin_event::PluginEvent,
    xplane::{
        dataref::collection::DataRefs,
        input_params::XPlaneInputParams,
        // input_params_consumer::{XPlaneDataRefUpdater, XPlaneInspectorUpdater},
        // input_params_debouncer::XPlaneParamDebouncer,
        // input_params_interpolator::XPlaneParamInterpolator,
        inspector::window::InspectorWindow,
        mapper::output::XPlaneSM2MOutputMapper,
        menu::{instance::PluginMenu, item::MenuItem},
        // output_params_supplier::XPlaneOutputSupplier,
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
    // input_chain: InputChain,
    // output_chain: OutputChain,
    input_metrics: Rc<RefCell<IOMetrics>>,
    output_metrics: Rc<RefCell<IOMetrics>>,
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
        let input_metrics = Rc::new(RefCell::new(IOMetrics::default()));
        let output_metrics = Rc::new(RefCell::new(IOMetrics::default()));
        let delta_supplier = Rc::new(RefCell::new(DeltaTimeSupplier::default()));

        // let input_chain = build_default_input_chain(
        //     datarefs.clone(),
        //     inspector.clone(),
        //     delta_supplier.clone(),
        //     input_metrics.clone(),
        //     output_metrics.clone(),
        // );

        Self {
            menu,
            datarefs: datarefs.clone(),
            inspector,
            rx,
            delta_supplier,
            // input_chain,
            // output_chain: build_default_output_chain(datarefs),
            input_metrics,
            output_metrics,
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
        // self.input_chain = self.build_generator_input_chain();
        self.menu.uncheck_item(MenuItem::Physics);
        self.menu.check_item(MenuItem::Inspector);
        self.datarefs.borrow_mut().general.disable_physics();
        self.inspector.borrow_mut().show();
    }

    fn stop_test(&mut self) {
        // self.input_chain = build_default_input_chain(
        //     self.datarefs.clone(),
        //     self.inspector.clone(),
        //     self.delta_supplier.clone(),
        //     self.input_metrics.clone(),
        //     self.output_metrics.clone(),
        // );
    }

    fn build_generator_input_chain(&self) /*-> InputChain*/
    {
        let data_refs = self.datarefs.borrow();
        let mut generator = USBParamGenerator::new(Duration::from_millis(20));
        let altitude = data_refs.location.coords().altitude.to_bits();
        generator.add_bounced(
            InputParamType::Altitude,
            altitude as i16,
            1,
            10,
            Duration::from_millis(40),
        );

        //     let input_params = self.datarefs.borrow().as_input();
        //     let params = XPlaneSM2MInputMapper::default().map(input_params);
        //     let mut generator = USBParamGenerator::dynamic(self.delta_supplier.clone());
        //     generator.update_params(&params);
        //     Chain::supply(generator)
        //         .map(SM2MXPlaneInputMapper::default())
        //         .map(XPlaneParamDebouncer::new())
        //         .map(XPlaneParamInterpolator::new(
        //             input_params,
        //             self.delta_supplier.clone(),
        //         ))
        //         .consume(XPlaneDataRefUpdater::new(self.datarefs.clone()))
        //         .consume(XPlaneInspectorUpdater::new(
        //             self.datarefs.clone(),
        //             self.inspector.clone(),
        //             self.input_metrics.clone(),
        //             self.output_metrics.clone(),
        //             self.delta_supplier.clone(),
        //         ))
    }

    fn execute(&mut self, delta: Duration) {
        self.delta_supplier.borrow_mut().update(delta);
        // self.input_chain.execute();
        // self.output_chain.execute();
    }
}

// fn build_default_input_chain(
//     datarefs: Rc<RefCell<DataRefs>>,
//     inspector: Rc<RefCell<InspectorWindow>>,
//     delta_supplier: Rc<RefCell<DeltaTimeSupplier>>,
//     input_metrics: Rc<RefCell<IOMetrics>>,
//     output_metrics: Rc<RefCell<IOMetrics>>,
// ) -> InputChain {
//     let input_params = datarefs.borrow().as_input();

//     Chain::supply(|| None)
//         .map(SM2MXPlaneInputMapper::default())
//         .map(XPlaneParamInterpolator::new(
//             input_params,
//             delta_supplier.clone(),
//         ))
//         .consume(XPlaneInspectorUpdater::new(
//             datarefs.clone(),
//             inspector.clone(),
//             input_metrics.clone(),
//             output_metrics.clone(),
//             delta_supplier.clone(),
//         ))
// }

// fn build_default_output_chain(datarefs: Rc<RefCell<DataRefs>>) -> OutputChain {
//     Chain::supply(XPlaneOutputSupplier::new(datarefs.clone())).map(XPlaneSM2MOutputMapper)
// }

impl FlightLoopCallback for Controller {
    fn flight_loop(&mut self, state: &mut LoopState) {
        self.handle_events();
        self.execute(state.since_last_call())
    }
}
