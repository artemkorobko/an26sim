#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum InputParamType {
    LatitudeHi,
    LatitudeLo,
    LongitudeHi,
    LongitudeLo,
    Altitude,
    Heading,
    Pitch,
    Roll,
    Ailerons,
    Elevator,
    Rudder,
    Flaps,
    EngineLeft,
    EngineRight,
    GearFront,
    GearLeft,
    GearRight,
    Lights,
    Reset,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InputParameter {
    pub ip_type: InputParamType,
    pub value: i16,
}

pub enum OutputParamType {
    TerrainAltitude,
}
