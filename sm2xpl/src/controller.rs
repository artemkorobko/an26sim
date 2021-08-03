use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use xplm::flight_loop::{FlightLoopCallback, LoopState};

use crate::{
    common::chain::{Chain, Mapper},
    io::{delta::DeltaTimeSupplier, generator::USBParamGenerator, index::output::*},
    plugin_event::PluginEvent,
    xplane::{
        consumer::{XPlaneDataRefUpdater, XPlaneInspectorUpdater},
        dataref::collection::DataRefs,
        debouncer::XPlaneParamDebouncer,
        inspector::window::InspectorWindow,
        mapper::{SM2MXPlaneMapper, XPlaneSM2MMapper},
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
        let ochain = build_default_output_chain(datarefs.clone(), inspector.clone(), delta.clone());

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
        let datarefs = self.datarefs.borrow();
        let params = XPlaneSM2MMapper::default().map(datarefs.as_output());
        let mut generator = USBParamGenerator::dynamic(self.delta_supplier.clone());
        fill_param_generator(&mut generator, &params);
        Chain::supply(generator)
            .map(SM2MXPlaneMapper::default())
            .map(XPlaneParamDebouncer::new(
                datarefs.as_input(),
                self.delta_supplier.clone(),
            ))
            .consume(XPlaneDataRefUpdater::new(self.datarefs.clone()))
    }

    fn execute(&mut self, delta: Duration) {
        if let Some(input) = &mut self.input_chain {
            self.delta_supplier.borrow_mut().update(delta);
            input.execute();
            self.output_chain.execute();
        }
    }
}

fn build_default_output_chain(
    datarefs: Rc<RefCell<DataRefs>>,
    inspector: Rc<RefCell<InspectorWindow>>,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
) -> OutputChain {
    Chain::supply(XPlaneOutputSupplier::new(datarefs))
        .consume(XPlaneInspectorUpdater::new(inspector, delta))
        .map(XPlaneSM2MMapper)
}

fn fill_param_generator(generator: &mut USBParamGenerator, params: &[u16]) {
    generator.set_latitude(17606, 21450);
    generator.set_longitude(3193, 15130);
    generator.set_altitude(params[ALT_IDX].reverse_bits());
    generator.set_heading(params[HDG_IDX].reverse_bits());
    generator.set_pitch(params[PITCH_IDX].reverse_bits());
    generator.set_roll(params[ROLL_IDX].reverse_bits());
    generator.set_ailerons(params[AIL_IDX].reverse_bits());
    generator.set_elevator(params[ELEV_IDX].reverse_bits());
    generator.set_rudder(params[RUD_IDX].reverse_bits());
    generator.set_flaps(params[FLP_IDX].reverse_bits());
    generator.set_engine_left(params[ENG_L_IDX].reverse_bits());
    generator.set_engine_right(params[ENG_R_IDX].reverse_bits());
    generator.set_gear_front(params[GEAR_F_IDX].reverse_bits());
    generator.set_gear_left(params[GEAR_L_IDX].reverse_bits());
    generator.set_gear_right(params[GEAR_R_IDX].reverse_bits());
    generator.set_lights(params[LIGHTS_IDX].reverse_bits());
}

impl FlightLoopCallback for Controller {
    fn flight_loop(&mut self, state: &mut LoopState) {
        self.handle_events();
        self.execute(state.since_last_call())
    }
}
