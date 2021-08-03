use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{inspector::rect_ext::RectExt, params::Lights},
};

use super::api::{update_widget, ApiResult};

pub struct LightsBlock {
    beacon: XPWidgetID,
    landing: XPWidgetID,
    navigation: XPWidgetID,
}

impl LightsBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let beacon = label!("Beacon:", parent, &rect);
        let rect = rect.to_next_line();
        let landing = label!("Landing:", parent, &rect);
        let rect = rect.to_next_line();
        let navigation = label!("Navigation:", parent, &rect);
        Ok((
            Self {
                beacon,
                landing,
                navigation,
            },
            rect,
        ))
    }

    pub fn update(&self, lights: &Lights) -> ApiResult<()> {
        update_widget(self.beacon, format_on_off(lights.beacon))?;
        update_widget(self.landing, format_on_off(lights.landing))?;
        update_widget(self.navigation, format_on_off(lights.navigation))
    }
}

fn format_on_off(value: bool) -> &'static str {
    match value {
        true => "on",
        false => "off",
    }
}
