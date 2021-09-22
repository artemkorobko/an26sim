use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::gears::GearsDataRef, inspector::rect_ext::RectExt},
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
        let front = create_label!("Gear front:", parent, rect);
        let rect = rect.to_next_line();
        let left = create_label!("Gear left:", parent, &rect);
        let rect = rect.to_next_line();
        let right = create_label!("Gear right:", parent, &rect);
        let block = Self { front, left, right };
        Ok((block, rect))
    }

    pub fn update(&self, gears: &GearsDataRef) -> ApiResult<()> {
        let min = 0.0;
        let max = 1.0;
        let state = gears.get();
        let percent = format_percent(state.front, min, max);
        update_widget(self.front, &percent)?;
        let percent = format_percent(state.left, min, max);
        update_widget(self.left, &percent)?;
        let percent = format_percent(state.right, min, max);
        update_widget(self.right, &percent)?;
        Ok(())
    }
}
