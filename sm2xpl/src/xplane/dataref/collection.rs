use xplm::data::borrowed::FindError;

use crate::xplane::params::{General, XPlaneInputParams, XPlaneOutputParams};

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

    pub fn set(&mut self, params: &XPlaneInputParams) {
        self.location.set(&params.location);
        self.orientation.set(&params.orientation);
        self.surfaces.set(&params.surfaces);
        self.engines.set(&params.engines);
        self.gears.set(&params.gears);
        self.lights.set(&params.lights);
    }

    pub fn as_input(&self) -> XPlaneInputParams {
        XPlaneInputParams {
            location: self.location.get(),
            orientation: self.orientation.get(),
            surfaces: self.surfaces.get(),
            engines: self.engines.get(),
            gears: self.gears.get(),
            lights: self.lights.get(),
            reset: false,
        }
    }

    pub fn as_output(&self) -> XPlaneOutputParams {
        let local = self.location.local();
        XPlaneOutputParams {
            terrain_distance: self.terrain_probe.distance(local.x, local.y, local.z),
        }
    }
}
