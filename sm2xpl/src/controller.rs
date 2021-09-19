use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use xplm::flight_loop::{FlightLoopCallback, LoopState};

use crate::{
    common::{delta::DeltaTimeSupplier, pipeline::Pipeline},
    io::{
        generator::{helper::ToGenerator, usb::USBParamGenerator},
        metrics::IOMetrics,
    },
    plugin_event::PluginEvent,
    xplane::{
        dataref::{
            collection::DataRefs, supplier::XPlaneOutputSupplier, updater::XPlaneDataRefUpdater,
        },
        debouncers::input::XPlaneParamDebouncer,
        input_params::XPlaneInputParams,
        inspector::{updater::XPlaneInspectorUpdater, window::InspectorWindow},
        interpolator::XPlaneParamInterpolator,
        mapper::{input::SM2MXPlaneInputMapper, output::XPlaneSM2MOutputMapper, transcoder},
        menu::{instance::PluginMenu, item::MenuItem},
    },
};

pub struct Controller {
    menu: Box<PluginMenu>,
    datarefs: Rc<RefCell<DataRefs>>,
    inspector: Rc<RefCell<InspectorWindow>>,
    rx: Receiver<PluginEvent>,
    delta_supplier: Rc<RefCell<DeltaTimeSupplier>>,
    input_pipeline: Pipeline<XPlaneInputParams>,
    output_pipeline: Pipeline<Vec<u8>>,
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

        let input_pipeline = build_default_input_pipeline(
            datarefs.clone(),
            inspector.clone(),
            delta_supplier.clone(),
            input_metrics.clone(),
            output_metrics.clone(),
        );

        Self {
            menu,
            datarefs: datarefs.clone(),
            inspector,
            rx,
            delta_supplier,
            input_pipeline,
            output_pipeline: build_default_output_pipeline(datarefs),
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
        self.input_pipeline = self.build_generator_pipeline();
        self.menu.uncheck_item(MenuItem::Physics);
        self.menu.check_item(MenuItem::Inspector);
        self.datarefs.borrow_mut().general.disable_physics();
        self.inspector.borrow_mut().show();
    }

    fn stop_test(&mut self) {
        self.input_pipeline = build_default_input_pipeline(
            self.datarefs.clone(),
            self.inspector.clone(),
            self.delta_supplier.clone(),
            self.input_metrics.clone(),
            self.output_metrics.clone(),
        );
    }

    fn build_generator_pipeline(&self) -> Pipeline<XPlaneInputParams> {
        let datarefs = self.datarefs.borrow();
        let params = XPlaneInputParams::from(datarefs);
        let generator = USBParamGenerator::from(self.delta_supplier.clone())
            .with_const(transcoder::latitude::encode(params.latitude).to_const_generator())
            .with_const(transcoder::longitude::encode(params.longitude).to_const_generator())
            .with_const(transcoder::altitude::encode(params.altitude).to_const_generator())
            .with_const(transcoder::heading::encode(params.heading).to_const_generator())
            .with_const(transcoder::pitch::encode(params.pitch).to_const_generator())
            .with_const(transcoder::roll::encode(params.roll).to_const_generator())
            .with_const(transcoder::ailerons::encode(params.ailerons).to_const_generator())
            .with_const(transcoder::elevator::encode(params.elevator).to_const_generator())
            .with_const(transcoder::rudder::encode(params.rudder).to_const_generator())
            .with_const(transcoder::flaps::encode(params.flaps).to_const_generator())
            .with_const(transcoder::engine::encode(params.engine_left).to_const_generator())
            .with_const(transcoder::engine::encode(params.engine_right).to_const_generator())
            .with_const(transcoder::gear::encode(params.gear_front).to_const_generator())
            .with_const(transcoder::gear::encode(params.gear_left).to_const_generator())
            .with_const(transcoder::gear::encode(params.gear_right).to_const_generator())
            .with_const(
                transcoder::light::encode(
                    params.light_landing,
                    params.light_navigation,
                    params.light_landing,
                )
                .to_const_generator(),
            )
            .with_const(transcoder::reset::encode(params.reset).to_const_generator())
            .delay(Duration::from_millis(20));

        Pipeline::supply(generator)
            .map(SM2MXPlaneInputMapper::default())
            .map(XPlaneParamDebouncer::new())
            .map(XPlaneParamInterpolator::new(
                params,
                self.delta_supplier.clone(),
            ))
            .consume(XPlaneDataRefUpdater::new(self.datarefs.clone()))
            .consume(XPlaneInspectorUpdater::new(
                self.datarefs.clone(),
                self.inspector.clone(),
                self.input_metrics.clone(),
                self.output_metrics.clone(),
                self.delta_supplier.clone(),
            ))
    }

    fn execute(&mut self, delta: Duration) {
        self.delta_supplier.borrow_mut().update(delta);
        self.input_pipeline.execute();
        self.output_pipeline.execute();
    }
}

fn build_default_input_pipeline(
    datarefs: Rc<RefCell<DataRefs>>,
    inspector: Rc<RefCell<InspectorWindow>>,
    delta_supplier: Rc<RefCell<DeltaTimeSupplier>>,
    input_metrics: Rc<RefCell<IOMetrics>>,
    output_metrics: Rc<RefCell<IOMetrics>>,
) -> Pipeline<XPlaneInputParams> {
    let params = XPlaneInputParams::from(datarefs.borrow());
    Pipeline::supply(|| None)
        .map(SM2MXPlaneInputMapper::default())
        .map(XPlaneParamInterpolator::new(params, delta_supplier.clone()))
        .consume(XPlaneInspectorUpdater::new(
            datarefs,
            inspector.clone(),
            input_metrics.clone(),
            output_metrics.clone(),
            delta_supplier.clone(),
        ))
}

fn build_default_output_pipeline(datarefs: Rc<RefCell<DataRefs>>) -> Pipeline<Vec<u8>> {
    Pipeline::supply(XPlaneOutputSupplier::new(datarefs.clone())).map(XPlaneSM2MOutputMapper)
}

impl FlightLoopCallback for Controller {
    fn flight_loop(&mut self, state: &mut LoopState) {
        self.handle_events();
        self.execute(state.since_last_call())
    }
}
