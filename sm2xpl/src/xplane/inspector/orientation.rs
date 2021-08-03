use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{inspector::rect_ext::RectExt, params::Orientation},
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
        let heading = label!("Heading:", parent, &rect);
        let rect = rect.to_next_line();
        let pitch = label!("Pitch:", parent, &rect);
        let rect = rect.to_next_line();
        let roll = label!("Roll:", parent, &rect);
        Ok((
            Self {
                heading,
                pitch,
                roll,
            },
            rect,
        ))
    }

    pub fn update(&self, orientation: &Orientation) -> ApiResult<()> {
        let value = format_degree(orientation.heading);
        update_widget(self.heading, &value)?;
        let value = format_degree(orientation.pitch);
        update_widget(self.pitch, &value)?;
        let value = format_degree(orientation.roll);
        update_widget(self.roll, &value)
    }
}
