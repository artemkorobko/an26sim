use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    label,
    xplane::{inspector::rect_ext::RectExt, params::Location},
};

use super::api::{update_widget, ApiResult};

pub struct LocationBlock {
    latitude: XPWidgetID,
    longitude: XPWidgetID,
    altitude: XPWidgetID,
    terrain: XPWidgetID,
}

impl LocationBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let latitude = label!("Latitude:", parent, &rect);
        let rect = rect.to_next_line();
        let longitude = label!("Longitude:", parent, &rect);
        let rect = rect.to_next_line();
        let altitude = label!("Altitude:", parent, &rect);
        let rect = rect.to_next_line();
        let terrain = label!("Terrain:", parent, &rect);
        Ok((
            Self {
                latitude,
                longitude,
                altitude,
                terrain,
            },
            rect,
        ))
    }

    pub fn update(&self, location: &Location, terrain: f32) -> ApiResult<()> {
        update_widget(self.latitude, &format_f64(location.latitude))?;
        update_widget(self.longitude, &format_f64(location.longitude))?;
        update_widget(self.altitude, &format_altitude(location.altitude))?;
        update_widget(self.terrain, &format_altitude(terrain as f64))
    }
}

fn format_f64(value: f64) -> String {
    format!("{:.6}", value)
}

fn format_altitude(value: f64) -> String {
    if value < 999.0 {
        format!("{:.2}m", value)
    } else {
        format!("{:.2}km", value / 1000.0)
    }
}
