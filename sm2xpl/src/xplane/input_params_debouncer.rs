use std::time::{Duration, Instant};

use crate::common::chain::Mapper;

use super::{
    debouncers::{
        angular::AngularDebouncer, boolean::BooleanDebouncer, generic::Debouncer,
        linear::LinearDebouncer,
    },
    input_params::{Engines, Gears, Lights, Location, Orientation, Surfaces, XPlaneInputParams},
};

pub struct XPlaneParamDebouncer {
    last_debounce: Instant,
    params: Option<DebouncedParams>,
}

impl XPlaneParamDebouncer {
    pub fn new() -> Self {
        Self {
            last_debounce: Instant::now(),
            params: None,
        }
    }

    fn debounce(&mut self, target: XPlaneInputParams) -> XPlaneInputParams {
        if let Some(params) = &mut self.params {
            let delta = Instant::now().duration_since(self.last_debounce);
            params.debounce(target, &delta)
        } else {
            let mut params = DebouncedParams::default();
            let output = params.assign(target);
            self.params = Some(params);
            output
        }
    }
}

impl Mapper<Option<XPlaneInputParams>, Option<XPlaneInputParams>> for XPlaneParamDebouncer {
    fn map(&mut self, input: Option<XPlaneInputParams>) -> Option<XPlaneInputParams> {
        input.map(|params| self.debounce(params))
    }
}

struct LocationDebouncer {
    latitude: LinearDebouncer<f64>,
    longitude: LinearDebouncer<f64>,
    altitude: LinearDebouncer<f64>,
}

impl Default for LocationDebouncer {
    fn default() -> Self {
        Self {
            latitude: LinearDebouncer::new(0.01),
            longitude: LinearDebouncer::new(0.01),
            altitude: LinearDebouncer::new(5.0),
        }
    }
}

impl Debouncer<Location> for LocationDebouncer {
    fn debounce(&mut self, target: Location, delta: &Duration) -> Location {
        Location {
            latitude: self.latitude.debounce(target.latitude, delta),
            longitude: self.longitude.debounce(target.longitude, delta),
            altitude: self.altitude.debounce(target.altitude, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Location {
        Location {
            latitude: self.latitude.integrate(delta),
            longitude: self.longitude.integrate(delta),
            altitude: self.altitude.integrate(delta),
        }
    }

    fn assign(&mut self, target: Location) -> Location {
        Location {
            latitude: self.latitude.assign(target.latitude),
            longitude: self.longitude.assign(target.longitude),
            altitude: self.altitude.assign(target.altitude),
        }
    }
}

struct OrientationDebouncer {
    heading: AngularDebouncer<f32>,
    pitch: AngularDebouncer<f32>,
    roll: AngularDebouncer<f32>,
}

impl Default for OrientationDebouncer {
    fn default() -> Self {
        Self {
            heading: AngularDebouncer::new(1.0),
            pitch: AngularDebouncer::new(1.0),
            roll: AngularDebouncer::new(5.0),
        }
    }
}

impl Debouncer<Orientation> for OrientationDebouncer {
    fn debounce(&mut self, target: Orientation, delta: &Duration) -> Orientation {
        Orientation {
            heading: self.heading.debounce(target.heading, delta),
            pitch: self.pitch.debounce(target.pitch, delta),
            roll: self.roll.debounce(target.roll, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Orientation {
        Orientation {
            heading: self.heading.integrate(delta),
            pitch: self.pitch.integrate(delta),
            roll: self.roll.integrate(delta),
        }
    }

    fn assign(&mut self, target: Orientation) -> Orientation {
        Orientation {
            heading: self.heading.assign(target.heading),
            pitch: self.pitch.assign(target.pitch),
            roll: self.roll.assign(target.roll),
        }
    }
}

struct SurfacesDebouncer {
    ailerons: LinearDebouncer<f32>,
    elevator: LinearDebouncer<f32>,
    rudder: LinearDebouncer<f32>,
    flaps: LinearDebouncer<f32>,
}

impl Default for SurfacesDebouncer {
    fn default() -> Self {
        Self {
            ailerons: LinearDebouncer::new(0.1),
            elevator: LinearDebouncer::new(0.1),
            rudder: LinearDebouncer::new(0.1),
            flaps: LinearDebouncer::new(0.1),
        }
    }
}

impl Debouncer<Surfaces> for SurfacesDebouncer {
    fn debounce(&mut self, target: Surfaces, delta: &Duration) -> Surfaces {
        Surfaces {
            ailerons: self.ailerons.debounce(target.ailerons, delta),
            elevator: self.elevator.debounce(target.elevator, delta),
            rudder: self.rudder.debounce(target.rudder, delta),
            flaps: self.flaps.debounce(target.flaps, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Surfaces {
        Surfaces {
            ailerons: self.ailerons.integrate(delta),
            elevator: self.elevator.integrate(delta),
            rudder: self.rudder.integrate(delta),
            flaps: self.flaps.integrate(delta),
        }
    }

    fn assign(&mut self, target: Surfaces) -> Surfaces {
        Surfaces {
            ailerons: self.ailerons.assign(target.ailerons),
            elevator: self.elevator.assign(target.elevator),
            rudder: self.rudder.assign(target.rudder),
            flaps: self.flaps.assign(target.flaps),
        }
    }
}

struct EnginesDebouncer {
    left: LinearDebouncer<f32>,
    right: LinearDebouncer<f32>,
}

impl Default for EnginesDebouncer {
    fn default() -> Self {
        Self {
            left: LinearDebouncer::new(10.0),
            right: LinearDebouncer::new(10.0),
        }
    }
}

impl Debouncer<Engines> for EnginesDebouncer {
    fn debounce(&mut self, target: Engines, delta: &Duration) -> Engines {
        Engines {
            left: self.left.debounce(target.left, delta),
            right: self.right.debounce(target.right, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Engines {
        Engines {
            left: self.left.integrate(delta),
            right: self.right.integrate(delta),
        }
    }

    fn assign(&mut self, target: Engines) -> Engines {
        Engines {
            left: self.left.assign(target.left),
            right: self.right.assign(target.right),
        }
    }
}

struct GearsDebouncer {
    front: LinearDebouncer<f32>,
    left: LinearDebouncer<f32>,
    right: LinearDebouncer<f32>,
}

impl Default for GearsDebouncer {
    fn default() -> Self {
        Self {
            front: LinearDebouncer::new(10.0),
            left: LinearDebouncer::new(10.0),
            right: LinearDebouncer::new(10.0),
        }
    }
}

impl Debouncer<Gears> for GearsDebouncer {
    fn debounce(&mut self, target: Gears, delta: &Duration) -> Gears {
        Gears {
            front: self.front.debounce(target.front, delta),
            left: self.left.debounce(target.left, delta),
            right: self.right.debounce(target.right, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Gears {
        Gears {
            front: self.front.integrate(delta),
            left: self.left.integrate(delta),
            right: self.right.integrate(delta),
        }
    }

    fn assign(&mut self, target: Gears) -> Gears {
        Gears {
            front: self.front.assign(target.front),
            left: self.left.assign(target.left),
            right: self.right.assign(target.right),
        }
    }
}

#[derive(Default)]
struct LightsDebouncer {
    landing: BooleanDebouncer,
    navigation: BooleanDebouncer,
    beacon: BooleanDebouncer,
}

impl Debouncer<Lights> for LightsDebouncer {
    fn debounce(&mut self, target: Lights, delta: &Duration) -> Lights {
        Lights {
            landing: self.landing.debounce(target.landing, delta),
            navigation: self.navigation.debounce(target.navigation, delta),
            beacon: self.beacon.debounce(target.beacon, delta),
        }
    }

    fn integrate(&mut self, delta: &Duration) -> Lights {
        Lights {
            landing: self.landing.integrate(delta),
            navigation: self.navigation.integrate(delta),
            beacon: self.beacon.integrate(delta),
        }
    }

    fn assign(&mut self, target: Lights) -> Lights {
        Lights {
            landing: self.landing.assign(target.landing),
            navigation: self.navigation.assign(target.navigation),
            beacon: self.beacon.assign(target.beacon),
        }
    }
}

#[derive(Default)]
struct DebouncedParams {
    location: LocationDebouncer,
    orientation: OrientationDebouncer,
    surfaces: SurfacesDebouncer,
    engines: EnginesDebouncer,
    gears: GearsDebouncer,
    lights: LightsDebouncer,
    reset: BooleanDebouncer,
}

impl Debouncer<XPlaneInputParams> for DebouncedParams {
    fn debounce(&mut self, target: XPlaneInputParams, delta: &Duration) -> XPlaneInputParams {
        let reset = self.reset.debounce(target.reset, delta);
        if reset {
            self.assign(target)
        } else {
            XPlaneInputParams {
                location: self.location.debounce(target.location, delta),
                orientation: self.orientation.debounce(target.orientation, delta),
                surfaces: self.surfaces.debounce(target.surfaces, delta),
                engines: self.engines.debounce(target.engines, delta),
                gears: self.gears.debounce(target.gears, delta),
                lights: target.lights,
                reset,
            }
        }
    }

    fn integrate(&mut self, delta: &Duration) -> XPlaneInputParams {
        XPlaneInputParams {
            location: self.location.integrate(delta),
            orientation: self.orientation.integrate(delta),
            surfaces: self.surfaces.integrate(delta),
            engines: self.engines.integrate(delta),
            gears: self.gears.integrate(delta),
            lights: self.lights.integrate(delta),
            reset: self.reset.integrate(delta),
        }
    }

    fn assign(&mut self, target: XPlaneInputParams) -> XPlaneInputParams {
        XPlaneInputParams {
            location: self.location.assign(target.location),
            orientation: self.orientation.assign(target.orientation),
            surfaces: self.surfaces.assign(target.surfaces),
            engines: self.engines.assign(target.engines),
            gears: self.gears.assign(target.gears),
            lights: self.lights.assign(target.lights),
            reset: self.reset.assign(target.reset),
        }
    }
}
