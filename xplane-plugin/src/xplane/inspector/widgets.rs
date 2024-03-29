use std::time::Duration;

use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{io::metrics::IOMetrics, xplane::dataref::collection::DataRefs};

use super::{
    api::ApiResult, engines::EnginesBlock, gears::GearsBlock, general::GeneralBlock, io::IOBlock,
    lights::LightsBlock, location::LocationBlock, orientation::OrientationBlock, rect_ext::RectExt,
    surfaces::SurfacesBlock, view::ViewBlock,
};

pub struct Widgets {
    general: GeneralBlock,
    io: IOBlock,
    surfaces: SurfacesBlock,
    lights: LightsBlock,
    view: ViewBlock,
    location: LocationBlock,
    orientation: OrientationBlock,
    engines: EnginesBlock,
    gears: GearsBlock,
}

impl Widgets {
    pub fn new(parent: XPWidgetID, window_rect: &Rect<i32>) -> ApiResult<Self> {
        let (general, rect) = GeneralBlock::new(parent, &window_rect.to_left_section())?;
        let (io, rect) = IOBlock::new(parent, &rect.to_next_block())?;
        let (lights, rect) = LightsBlock::new(parent, &rect.to_next_block())?;
        let (view, _) = ViewBlock::new(parent, &rect.to_next_block())?;
        let (location, rect) = LocationBlock::new(parent, &window_rect.to_right_section())?;
        let (orientation, rect) = OrientationBlock::new(parent, &rect.to_next_block())?;
        let (engines, rect) = EnginesBlock::new(parent, &rect.to_next_block())?;
        let (gears, rect) = GearsBlock::new(parent, &rect.to_next_block())?;
        let (surfaces, _) = SurfacesBlock::new(parent, &rect.to_next_block())?;
        Ok(Self {
            general,
            io,
            surfaces,
            lights,
            view,
            location,
            orientation,
            engines,
            gears,
        })
    }

    pub fn update(
        &self,
        data_refs: &DataRefs,
        input: &mut IOMetrics,
        output: &mut IOMetrics,
        delta: &Duration,
    ) -> ApiResult<()> {
        self.general.update(&data_refs.general)?;
        self.io.update(input, output, delta)?;
        self.surfaces.update(&data_refs.surfaces)?;
        self.lights.update(&data_refs.lights)?;
        self.view.update(&data_refs.view)?;
        self.location.update(&data_refs.location)?;
        self.orientation.update(&data_refs.orientation)?;
        self.engines.update(&data_refs.engines)?;
        self.gears.update(&data_refs.gears)
    }
}
