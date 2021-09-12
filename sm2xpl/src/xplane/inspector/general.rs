use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{
        dataref::variables::general::GeneralDataRef,
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
        let fps = create_label!("FPS:", parent, &rect);
        let rect = rect.to_next_line();
        let physics = create_label!("Physics:", parent, &rect);
        let block = Self { fps, physics };
        Ok((block, rect))
    }

    pub fn update(&self, general: &GeneralDataRef) -> ApiResult<()> {
        update_widget(self.fps, &format!("{:.2}", general.fps()))?;
        update_widget(self.physics, format_enabled(general.is_physics_enabled()))?;
        Ok(())
    }
}

fn format_enabled(state: bool) -> &'static str {
    match state {
        true => "enabled",
        false => "disabled",
    }
}
