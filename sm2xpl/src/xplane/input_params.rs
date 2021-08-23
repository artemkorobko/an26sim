#[derive(Default, Debug, Copy, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Orientation {
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Surfaces {
    pub ailerons: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Engines {
    pub left: f32,
    pub right: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Gears {
    pub front: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Lights {
    pub landing: bool,
    pub navigation: bool,
    pub beacon: bool,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct XPlaneInputParams {
    pub location: Location,
    pub orientation: Orientation,
    pub surfaces: Surfaces,
    pub engines: Engines,
    pub gears: Gears,
    pub lights: Lights,
    pub reset: bool,
}

#[derive(Default, Copy, Clone)]
pub struct General {
    pub fps: f32,
    pub physics: bool,
}

#[derive(Default, Copy, Clone)]
pub struct View {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
