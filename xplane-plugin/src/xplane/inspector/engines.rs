use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::engines::EnginesDataRef, inspector::rect_ext::RectExt},
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
        let left = create_label!("Engine left:", parent, rect);
        let rect = rect.to_next_line();
        let right = create_label!("Engine right:", parent, &rect);
        let block = Self { left, right };
        Ok((block, rect))
    }

    pub fn update(&self, engines: &EnginesDataRef) -> ApiResult<()> {
        let min = 0.0;
        let max = 166.0;
        let state = engines.get();
        let percent = format_percent(state.left, min, max);
        update_widget(self.left, &percent)?;
        let percent = format_percent(state.right, min, max);
        update_widget(self.right, &percent)?;
        Ok(())
    }
}
