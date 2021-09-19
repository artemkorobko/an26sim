use xplm::data::borrowed::FindError;
use xplm_sys::{
    xplm_ProbeHitTerrain, XPLMCreateProbe, XPLMProbeInfo_t, XPLMProbeRef, XPLMProbeResult,
    XPLMProbeTerrainXYZ, XPLMProbeType,
};

pub struct TerrainProbe {
    id: XPLMProbeRef,
    cabin_alt: f32,
}

impl TerrainProbe {
    pub fn new(cabin_alt: f32) -> Result<Self, FindError> {
        const PROBE_Y: XPLMProbeType = xplm_ProbeHitTerrain as XPLMProbeType;
        let dataref = unsafe { XPLMCreateProbe(PROBE_Y) };
        if dataref.is_null() {
            return Err(FindError::NotFound);
        } else {
            Ok(Self {
                id: dataref,
                cabin_alt,
            })
        }
    }

    pub fn distance(&self, x: f64, y: f64, z: f64) -> f32 {
        let x = x as f32;
        let y = y as f32;
        let z = z as f32;
        let mut info = XPLMProbeInfo_t {
            structSize: std::mem::size_of::<XPLMProbeInfo_t>() as i32,
            locationX: 0.0,
            locationY: 0.0,
            locationZ: 0.0,
            normalX: 0.0,
            normalY: 0.0,
            normalZ: 0.0,
            velocityX: 0.0,
            velocityY: 0.0,
            velocityZ: 0.0,
            is_wet: 0,
        };

        let result = unsafe { XPLMProbeTerrainXYZ(self.id, x, y, z, &mut info) };
        const PROBE_RESULT_TERRAIN: XPLMProbeResult = xplm_ProbeHitTerrain as XPLMProbeResult;
        if result == PROBE_RESULT_TERRAIN {
            let distance = y - info.locationY - self.cabin_alt;
            distance
        } else {
            0.0
        }
    }
}
