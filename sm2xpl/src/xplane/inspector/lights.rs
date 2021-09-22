use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::lights::LightsDataRef, inspector::rect_ext::RectExt},
};

use super::api::{update_widget, ApiResult};

pub struct LightsBlock {
    beacon: XPWidgetID,
    landing: XPWidgetID,
    navigation: XPWidgetID,
}

impl LightsBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let beacon = create_label!("Beacon:", parent, &rect);
        let rect = rect.to_next_line();
        let landing = create_label!("Landing:", parent, &rect);
        let rect = rect.to_next_line();
        let navigation = create_label!("Navigation:", parent, &rect);
        let block = Self {
            beacon,
            landing,
            navigation,
        };
        Ok((block, rect))
    }

    pub fn update(&self, lights: &LightsDataRef) -> ApiResult<()> {
        update_widget(self.beacon, format_on_off(lights.beacon()))?;
        update_widget(self.landing, format_on_off(lights.landing()))?;
        update_widget(self.navigation, format_on_off(lights.navigation()))?;
        Ok(())
    }
}

fn format_on_off(value: bool) -> &'static str {
    match value {
        true => "on",
        false => "off",
    }
}
