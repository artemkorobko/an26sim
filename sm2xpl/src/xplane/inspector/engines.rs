use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{input_params::Engines, inspector::rect_ext::RectExt},
};

use super::{
    api::{update_widget, ApiResult},
    helper::format_percent,
};

pub struct EnginesBlock {
    left: XPWidgetID,
    right: XPWidgetID,
}

impl EnginesBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let left = label!("Engine left:", parent, &rect);
        let rect = rect.to_next_line();
        let right = label!("Engine right:", parent, &rect);
        Ok((Self { left, right }, rect))
    }

    pub fn update(&self, engines: &Engines) -> ApiResult<()> {
        let min = 0.0;
        let max = 166.0;
        let percent = format_percent(engines.left, min, max);
        update_widget(self.left, &percent)?;
        let percent = format_percent(engines.right, min, max);
        update_widget(self.right, &percent)
    }
}
