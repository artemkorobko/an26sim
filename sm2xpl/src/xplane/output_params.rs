#[derive(Default, Copy, Clone)]
pub struct XPlaneOutputParams {
    pub terrain_distance: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub ailerons: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
    pub engine_left: f32,
    pub engine_right: f32,
    pub gear_front: f32,
    pub gear_left: f32,
    pub gear_right: f32,
    pub landing: bool,
    pub navigation: bool,
    pub beacon: bool,
    pub reset: bool,
}
