use std::time::Instant;

use crate::{common::pipeline::Mapper, xplane::input_params::XPlaneInputParams};

use super::{
    angular::AngularDebouncer, boolean::BooleanDebouncer, generic::Debouncer,
    linear::LinearDebouncer,
};

enum DebouncerState {
    Initial,
    Prepared,
}

pub struct XPlaneParamDebouncer {
    last_debounce: Instant,
    latitude: LinearDebouncer<f64>,
    longitude: LinearDebouncer<f64>,
    altitude: LinearDebouncer<f64>,
    heading: AngularDebouncer<f32>,
    pitch: AngularDebouncer<f32>,
    roll: AngularDebouncer<f32>,
    ailerons: LinearDebouncer<f32>,
    elevator: LinearDebouncer<f32>,
    rudder: LinearDebouncer<f32>,
    flaps: LinearDebouncer<f32>,
    engine_left: LinearDebouncer<f32>,
    engine_right: LinearDebouncer<f32>,
    gear_front: LinearDebouncer<f32>,
    gear_left: LinearDebouncer<f32>,
    gear_right: LinearDebouncer<f32>,
    light_landing: BooleanDebouncer,
    light_navigation: BooleanDebouncer,
    light_beacon: BooleanDebouncer,
    reset: BooleanDebouncer,
    state: DebouncerState,
}

impl XPlaneParamDebouncer {
    pub fn new() -> Self {
        Self {
            last_debounce: Instant::now(),
            latitude: LinearDebouncer::new(0.01),
            longitude: LinearDebouncer::new(0.01),
            altitude: LinearDebouncer::new(5.0),
            heading: AngularDebouncer::new(1.0),
            pitch: AngularDebouncer::new(1.0),
            roll: AngularDebouncer::new(5.0),
            ailerons: LinearDebouncer::new(0.1),
            elevator: LinearDebouncer::new(0.1),
            rudder: LinearDebouncer::new(0.1),
            flaps: LinearDebouncer::new(0.1),
            engine_left: LinearDebouncer::new(10.0),
            engine_right: LinearDebouncer::new(10.0),
            gear_front: LinearDebouncer::new(10.0),
            gear_left: LinearDebouncer::new(10.0),
            gear_right: LinearDebouncer::new(10.0),
            light_landing: BooleanDebouncer::default(),
            light_navigation: BooleanDebouncer::default(),
            light_beacon: BooleanDebouncer::default(),
            reset: BooleanDebouncer::default(),
            state: DebouncerState::Initial,
        }
    }

    fn reset(&mut self, params: XPlaneInputParams) {
        self.latitude.assign(params.latitude);
        self.longitude.assign(params.longitude);
        self.altitude.assign(params.altitude);
        self.heading.assign(params.heading);
        self.pitch.assign(params.pitch);
        self.roll.assign(params.roll);
        self.ailerons.assign(params.ailerons);
        self.elevator.assign(params.elevator);
        self.rudder.assign(params.rudder);
        self.flaps.assign(params.flaps);
        self.engine_left.assign(params.engine_left);
        self.engine_right.assign(params.engine_right);
        self.gear_front.assign(params.gear_front);
        self.gear_left.assign(params.gear_left);
        self.gear_right.assign(params.gear_right);
        self.light_landing.assign(params.light_landing);
        self.light_navigation.assign(params.light_navigation);
        self.light_beacon.assign(params.light_beacon);
        self.reset.assign(params.reset);
    }

    fn debounce(&mut self, params: XPlaneInputParams) -> XPlaneInputParams {
        let now = Instant::now();
        let delta = now.duration_since(self.last_debounce);
        self.last_debounce = now;
        XPlaneInputParams {
            latitude: self.latitude.debounce(params.latitude, &delta),
            longitude: self.longitude.debounce(params.longitude, &delta),
            altitude: self.altitude.debounce(params.altitude, &delta),
            heading: self.heading.debounce(params.heading, &delta),
            pitch: self.pitch.debounce(params.pitch, &delta),
            roll: self.roll.debounce(params.roll, &delta),
            ailerons: self.ailerons.debounce(params.ailerons, &delta),
            elevator: self.elevator.debounce(params.elevator, &delta),
            rudder: self.rudder.debounce(params.rudder, &delta),
            flaps: self.flaps.debounce(params.flaps, &delta),
            engine_left: self.engine_left.debounce(params.engine_left, &delta),
            engine_right: self.engine_right.debounce(params.engine_right, &delta),
            gear_front: self.gear_front.debounce(params.gear_front, &delta),
            gear_left: self.gear_left.debounce(params.gear_left, &delta),
            gear_right: self.gear_right.debounce(params.gear_right, &delta),
            light_landing: self.light_landing.debounce(params.light_landing, &delta),
            light_navigation: self
                .light_navigation
                .debounce(params.light_navigation, &delta),
            light_beacon: self.light_beacon.debounce(params.light_beacon, &delta),
            reset: self.reset.debounce(params.reset, &delta),
        }
    }
}

impl Mapper<Option<XPlaneInputParams>, Option<XPlaneInputParams>> for XPlaneParamDebouncer {
    fn map(&mut self, input: Option<XPlaneInputParams>) -> Option<XPlaneInputParams> {
        if let Some(params) = input {
            let result = match self.state {
                DebouncerState::Initial => {
                    self.reset(params);
                    self.state = DebouncerState::Prepared;
                    self.debounce(params)
                }
                DebouncerState::Prepared => self.debounce(params),
            };
            Some(result)
        } else {
            None
        }
    }
}
