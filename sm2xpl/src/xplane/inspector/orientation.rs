use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::orientation::OrientationDataRef, inspector::rect_ext::RectExt},
};

use super::{
    api::{update_widget, ApiResult},
    helper::format_degree,
};

pub struct OrientationBlock {
    heading: XPWidgetID,
    pitch: XPWidgetID,
    roll: XPWidgetID,
}

impl OrientationBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let heading = create_label!("Heading:", parent, rect);
        let rect = rect.to_next_line();
        let pitch = create_label!("Pitch:", parent, &rect);
        let rect = rect.to_next_line();
        let roll = create_label!("Roll:", parent, &rect);
        let block = Self {
            heading,
            pitch,
            roll,
        };
        Ok((block, rect))
    }

    pub fn update(&self, orientation: &OrientationDataRef) -> ApiResult<()> {
        let value = format_degree(orientation.heading());
        update_widget(self.heading, &value)?;
        let value = format_degree(orientation.pitch());
        update_widget(self.pitch, &value)?;
        let value = format_degree(orientation.roll());
        update_widget(self.roll, &value)?;
        Ok(())
    }
}
