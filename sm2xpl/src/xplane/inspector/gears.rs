use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{input_params::Gears, inspector::rect_ext::RectExt},
};

use super::{
    api::{update_widget, ApiResult},
    helper::format_percent,
};

pub struct GearsBlock {
    front: XPWidgetID,
    left: XPWidgetID,
    right: XPWidgetID,
}

impl GearsBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let front = label!("Gear front:", parent, &rect);
        let rect = rect.to_next_line();
        let left = label!("Gear left:", parent, &rect);
        let rect = rect.to_next_line();
        let right = label!("Gear right:", parent, &rect);
        Ok((Self { front, left, right }, rect))
    }

    pub fn update(&self, gears: &Gears) -> ApiResult<()> {
        let min = 0.0;
        let max = 1.0;
        let percent = format_percent(gears.front, min, max);
        update_widget(self.front, &percent)?;
        let percent = format_percent(gears.left, min, max);
        update_widget(self.left, &percent)?;
        let percent = format_percent(gears.right, min, max);
        update_widget(self.right, &percent)
    }
}
