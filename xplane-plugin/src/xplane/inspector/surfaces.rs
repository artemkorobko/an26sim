use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::surfaces::SurfacesDataRef, inspector::rect_ext::RectExt},
};

use super::{
    api::{update_widget, ApiResult},
    helper::format_percent,
};

pub struct SurfacesBlock {
    ailerons: XPWidgetID,
    elevator: XPWidgetID,
    rudder: XPWidgetID,
    flaps: XPWidgetID,
}

impl SurfacesBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let ailerons = create_label!("Ailerons:", parent, rect);
        let rect = rect.to_next_line();
        let elevator = create_label!("Elevator:", parent, &rect);
        let rect = rect.to_next_line();
        let rudder = create_label!("Rudder:", parent, &rect);
        let rect = rect.to_next_line();
        let flaps = create_label!("Flaps:", parent, &rect);
        let block = Self {
            ailerons,
            elevator,
            rudder,
            flaps,
        };
        Ok((block, rect))
    }

    pub fn update(&self, surfaces: &SurfacesDataRef) -> ApiResult<()> {
        let percent = format_percent(surfaces.ailerons(), -1.0, 1.0);
        update_widget(self.ailerons, &percent)?;
        let percent = format_percent(surfaces.elevator(), -1.0, 1.0);
        update_widget(self.elevator, &percent)?;
        let percent = format_percent(surfaces.rudder(), -1.0, 1.0);
        update_widget(self.rudder, &percent)?;
        let percent = format_percent(surfaces.flaps(), 0.0, 1.0);
        update_widget(self.flaps, &percent)?;
        Ok(())
    }
}
