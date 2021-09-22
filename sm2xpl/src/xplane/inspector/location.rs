use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{dataref::variables::location::LocationDataRef, inspector::rect_ext::RectExt},
};

use super::api::{update_widget, ApiResult};

pub struct LocationBlock {
    latitude: XPWidgetID,
    longitude: XPWidgetID,
    altitude: XPWidgetID,
    agl: XPWidgetID,
}

impl LocationBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let latitude = create_label!("Latitude:", parent, &rect);
        let rect = rect.to_next_line();
        let longitude = create_label!("Longitude:", parent, &rect);
        let rect = rect.to_next_line();
        let altitude = create_label!("Altitude:", parent, &rect);
        let rect = rect.to_next_line();
        let agl = create_label!("AGL:", parent, &rect);
        let block = Self {
            latitude,
            longitude,
            altitude,
            agl,
        };
        Ok((block, rect))
    }

    pub fn update(&self, location: &LocationDataRef) -> ApiResult<()> {
        let coords = location.coords();
        update_widget(self.latitude, &format_f64(coords.latitude))?;
        update_widget(self.longitude, &format_f64(coords.longitude))?;
        update_widget(self.altitude, &format_altitude(coords.altitude))?;
        update_widget(self.agl, &format_altitude(location.agl() as f64))?;
        Ok(())
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
