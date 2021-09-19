use std::{cell::RefCell, rc::Rc, sync::mpsc::Receiver, time::Duration};

use xplm::flight_loop::{FlightLoopCallback, LoopState};

use crate::{
    common::{
        delta::DeltaTimeSupplier,
        percent::Percent,
        pipeline::{Mapper, Pipeline},
    },
    io::{
        generator::{helper::ToGenerator, usb::USBParamGenerator},
        metrics::IOMetrics,
    },
    plugin_event::PluginEvent,
    xplane::{
        dataref::collection::DataRefs,
        input_params::XPlaneInputParams,
        inspector::window::InspectorWindow,
        mapper::{
            input::SM2MXPlaneInputMapper, output::XPlaneSM2MOutputMapper as OutMap, transcoder,
        },
        menu::{instance::PluginMenu, item::MenuItem},
    },
};

type InputPipeline = Pipeline<XPlaneInputParams>;
type OutputPipeline = Pipeline<Vec<u16>>;

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

    in_test: bool,
    latitude_hi: u16,
    latitude_lo: u16,
    latitude_delay: Duration,
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

            in_test: false,
            latitude_hi: u16::MIN,
            latitude_lo: u16::MIN,
            latitude_delay: Duration::ZERO,
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

        self.in_test = true;
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

        Pipeline::supply(generator).map(SM2MXPlaneInputMapper::default());
        // .map(XPlaneParamDebouncer::new());
        //     .map(XPlaneParamInterpolator::new(
        //         input_params,
        //         self.delta_supplier.clone(),
        //     ))
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

        if self.in_test {
            self.latitude_delay += delta;
            let (value, overflow) = self.latitude_lo.overflowing_add(50);
            if !overflow {
                self.latitude_lo = value;
                let latitude_hi = ((self.latitude_hi as u32) << 16) & 0xffff0000;
                let latitude_lo = (self.latitude_lo as u32) & 0xffff;
                let latitude = latitude_hi | latitude_lo;
                let latitude = (latitude as f64).scale(u32::MIN as f64, u32::MAX as f64, 0.0, 90.0);
                println!(
                    "{:#07} {:#07} ({:#034b}) = {}",
                    self.latitude_hi, self.latitude_lo, self.latitude_lo, latitude
                );

                // let lat_hi = (self.latitude_hi as f64).scale(0.0, 32767.0, 0.0, 90.0);
                // let lat_lo = (self.latitude_lo as f64).scale(-32767.0, 32767.0, 0.0, 0.0027465);
                // let latitude = lat_hi + lat_lo;
                // println!("{} {} = {}", self.latitude_hi, self.latitude_lo, latitude);

                self.datarefs
                    .borrow_mut()
                    .location
                    .set_coords(latitude, 0.0, 100.0);
            } else {
                println!("HI + 1");
                self.latitude_hi += 1;
                self.latitude_lo = u16::MIN;
            }
        }
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
