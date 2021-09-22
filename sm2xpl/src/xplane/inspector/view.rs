use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::view::ViewDataRef, inspector::rect_ext::RectExt},
};

use super::api::{update_widget, ApiResult};

pub struct ViewBlock {
    x: XPWidgetID,
    y: XPWidgetID,
    z: XPWidgetID,
}

impl ViewBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let x = create_label!("View X:", parent, &rect);
        let rect = rect.to_next_line();
        let y = create_label!("View Y:", parent, &rect);
        let rect = rect.to_next_line();
        let z = create_label!("View Z:", parent, &rect);
        let block = Self { x, y, z };
        Ok((block, rect))
    }

    pub fn update(&self, view: &ViewDataRef) -> ApiResult<()> {
        update_widget(self.x, &format_f32(view.x()))?;
        update_widget(self.y, &format_f32(view.y()))?;
        update_widget(self.z, &format_f32(view.z()))?;
        Ok(())
    }
}

fn format_f32(value: f32) -> String {
    format!("{:.2}", value)
}
