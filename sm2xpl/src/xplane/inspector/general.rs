use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{
        input_params::General,
        inspector::{api::update_widget, rect_ext::RectExt},
    },
};

use super::api::ApiResult;

pub struct GeneralBlock {
    fps: XPWidgetID,
    physics: XPWidgetID,
}

impl GeneralBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let fps = label!("FPS:", parent, &rect);
        let rect = rect.to_next_line();
        let physics = label!("Physics:", parent, &rect);
        Ok((Self { fps, physics }, rect))
    }

    pub fn update(&self, general: &General) -> ApiResult<()> {
        update_widget(self.fps, &format!("{:.2}", general.fps))?;
        update_widget(self.physics, format_enabled(general.physics))
    }
}

fn format_enabled(state: bool) -> &'static str {
    match state {
        true => "enabled",
        false => "disabled",
    }
}
