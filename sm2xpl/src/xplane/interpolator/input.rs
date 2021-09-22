use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{
        delta::DeltaTimeSupplier,
        pipeline::{Mapper, Supplier},
    },
    xplane::input_params::XPlaneInputParams,
};

use super::{generic::Interpolator, transparent::TransparentInterpolator};

pub struct XPlaneParamInterpolator {
    latitude: TransparentInterpolator<f64>,
    longitude: TransparentInterpolator<f64>,
    altitude: TransparentInterpolator<f64>,
    heading: TransparentInterpolator<f32>,
    pitch: TransparentInterpolator<f32>,
    roll: TransparentInterpolator<f32>,
    ailerons: TransparentInterpolator<f32>,
    elevator: TransparentInterpolator<f32>,
    rudder: TransparentInterpolator<f32>,
    flaps: TransparentInterpolator<f32>,
    engine_left: TransparentInterpolator<f32>,
    engine_right: TransparentInterpolator<f32>,
    gear_front: TransparentInterpolator<f32>,
    gear_left: TransparentInterpolator<f32>,
    gear_right: TransparentInterpolator<f32>,
    light_landing: bool,
    light_navigation: bool,
    light_beacon: bool,
    reset: bool,
    delta: Rc<RefCell<DeltaTimeSupplier>>,
}

impl XPlaneParamInterpolator {
    pub fn new(default: XPlaneInputParams, delta: Rc<RefCell<DeltaTimeSupplier>>) -> Self {
        Self {
            latitude: TransparentInterpolator::new(default.latitude),
            longitude: TransparentInterpolator::new(default.longitude),
            altitude: TransparentInterpolator::new(default.altitude),
            heading: TransparentInterpolator::new(default.heading),
            pitch: TransparentInterpolator::new(default.pitch),
            roll: TransparentInterpolator::new(default.roll),
            ailerons: TransparentInterpolator::new(default.ailerons),
            elevator: TransparentInterpolator::new(default.elevator),
            rudder: TransparentInterpolator::new(default.rudder),
            flaps: TransparentInterpolator::new(default.flaps),
            engine_left: TransparentInterpolator::new(default.engine_left),
            engine_right: TransparentInterpolator::new(default.engine_right),
            gear_front: TransparentInterpolator::new(default.gear_front),
            gear_left: TransparentInterpolator::new(default.gear_left),
            gear_right: TransparentInterpolator::new(default.gear_right),
            light_landing: default.light_landing,
            light_navigation: default.light_navigation,
            light_beacon: default.light_beacon,
            reset: default.reset,
            delta,
        }
    }

    fn reset(&mut self, params: XPlaneInputParams) {
        self.latitude.reset(params.latitude);
        self.longitude.reset(params.longitude);
        self.altitude.reset(params.altitude);
        self.heading.reset(params.heading);
        self.pitch.reset(params.pitch);
        self.roll.reset(params.roll);
        self.ailerons.reset(params.ailerons);
        self.elevator.reset(params.elevator);
        self.rudder.reset(params.rudder);
        self.flaps.reset(params.flaps);
        self.engine_left.reset(params.engine_left);
        self.engine_right.reset(params.engine_right);
        self.gear_front.reset(params.gear_front);
        self.gear_left.reset(params.gear_left);
        self.gear_right.reset(params.gear_right);
        self.light_landing = params.light_landing;
        self.light_navigation = params.light_navigation;
        self.light_beacon = params.light_beacon;
        self.reset = params.reset;
    }

    fn update(&mut self, params: XPlaneInputParams) {
        self.latitude.update(params.latitude);
        self.longitude.update(params.longitude);
        self.altitude.update(params.altitude);
        self.heading.update(params.heading);
        self.pitch.update(params.pitch);
        self.roll.update(params.roll);
        self.ailerons.update(params.ailerons);
        self.elevator.update(params.elevator);
        self.rudder.update(params.rudder);
        self.flaps.update(params.flaps);
        self.engine_left.update(params.engine_left);
        self.engine_right.update(params.engine_right);
        self.gear_front.update(params.gear_front);
        self.gear_left.update(params.gear_left);
        self.gear_right.update(params.gear_right);
        self.light_landing = params.light_landing;
        self.light_navigation = params.light_navigation;
        self.light_beacon = params.light_beacon;
        self.reset = params.reset;
    }

    fn interpolate(&mut self) -> XPlaneInputParams {
        let delta = self.delta.borrow_mut().supply();
        XPlaneInputParams {
            latitude: self.latitude.interpolate(&delta),
            longitude: self.longitude.interpolate(&delta),
            altitude: self.altitude.interpolate(&delta),
            heading: self.heading.interpolate(&delta),
            pitch: self.pitch.interpolate(&delta),
            roll: self.roll.interpolate(&delta),
            ailerons: self.ailerons.interpolate(&delta),
            elevator: self.elevator.interpolate(&delta),
            rudder: self.rudder.interpolate(&delta),
            flaps: self.flaps.interpolate(&delta),
            engine_left: self.engine_left.interpolate(&delta),
            engine_right: self.engine_right.interpolate(&delta),
            gear_front: self.gear_front.interpolate(&delta),
            gear_left: self.gear_left.interpolate(&delta),
            gear_right: self.gear_right.interpolate(&delta),
            light_landing: self.light_landing,
            light_navigation: self.light_navigation,
            light_beacon: self.light_beacon,
            reset: self.reset,
        }
    }
}

impl Mapper<Option<XPlaneInputParams>, XPlaneInputParams> for XPlaneParamInterpolator {
    fn map(&mut self, input: Option<XPlaneInputParams>) -> XPlaneInputParams {
        if let Some(params) = input {
            if params.reset {
                self.reset(params);
            } else {
                self.update(params);
            }
        }

        self.interpolate()
    }
}
