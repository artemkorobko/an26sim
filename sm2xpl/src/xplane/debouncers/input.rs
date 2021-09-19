use std::time::Instant;

use crate::{common::pipeline::Mapper, xplane::input_params::XPlaneInputParams};

use super::{
    angular::AngularDebouncer, boolean::BooleanDebouncer, generic::Debouncer,
    linear::LinearDebouncer,
};

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
        }
    }

    fn debounce(&mut self, target: XPlaneInputParams) -> XPlaneInputParams {
        let now = Instant::now();
        let delta = now.duration_since(self.last_debounce);
        self.last_debounce = now;
        XPlaneInputParams {
            latitude: self.latitude.debounce(target.latitude, &delta),
            longitude: self.longitude.debounce(target.longitude, &delta),
            altitude: self.altitude.debounce(target.altitude, &delta),
            heading: self.heading.debounce(target.heading, &delta),
            pitch: self.pitch.debounce(target.pitch, &delta),
            roll: self.roll.debounce(target.roll, &delta),
            ailerons: self.ailerons.debounce(target.ailerons, &delta),
            elevator: self.elevator.debounce(target.elevator, &delta),
            rudder: self.rudder.debounce(target.rudder, &delta),
            flaps: self.flaps.debounce(target.flaps, &delta),
            engine_left: self.engine_left.debounce(target.engine_left, &delta),
            engine_right: self.engine_right.debounce(target.engine_right, &delta),
            gear_front: self.gear_front.debounce(target.gear_front, &delta),
            gear_left: self.gear_left.debounce(target.gear_left, &delta),
            gear_right: self.gear_right.debounce(target.gear_right, &delta),
            light_landing: self.light_landing.debounce(target.light_landing, &delta),
            light_navigation: self
                .light_navigation
                .debounce(target.light_navigation, &delta),
            light_beacon: self.light_beacon.debounce(target.light_beacon, &delta),
            reset: self.reset.debounce(target.reset, &delta),
        }
    }
}

impl Mapper<Option<XPlaneInputParams>, Option<XPlaneInputParams>> for XPlaneParamDebouncer {
    fn map(&mut self, input: Option<XPlaneInputParams>) -> Option<XPlaneInputParams> {
        input.map(|params| self.debounce(params))
    }
}
