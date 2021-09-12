use xplm::data::borrowed::FindError;

use super::variables::{
    engines::EnginesDataRef, gears::GearsDataRef, general::GeneralDataRef, lights::LightsDataRef,
    location::LocationDataRef, orientation::OrientationDataRef, surfaces::SurfacesDataRef,
    terrain_probe::TerrainProbe, view::ViewDataRef,
};

pub struct DataRefs {
    pub general: GeneralDataRef,
    pub location: LocationDataRef,
    pub orientation: OrientationDataRef,
    pub surfaces: SurfacesDataRef,
    pub engines: EnginesDataRef,
    pub gears: GearsDataRef,
    pub lights: LightsDataRef,
    pub view: ViewDataRef,
    pub terrain_probe: TerrainProbe,
}

impl DataRefs {
    pub fn new(cabin_alt: f32) -> Result<Self, FindError> {
        Ok(Self {
            general: GeneralDataRef::new()?,
            location: LocationDataRef::new()?,
            orientation: OrientationDataRef::new()?,
            surfaces: SurfacesDataRef::new()?,
            engines: EnginesDataRef::new()?,
            gears: GearsDataRef::new()?,
            lights: LightsDataRef::new()?,
            view: ViewDataRef::new()?,
            terrain_probe: TerrainProbe::new(cabin_alt)?,
        })
    }
}
