use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    xplane::{
        dataref::variables::{location::LocationDataRef, terrain_probe::TerrainProbe},
        inspector::rect_ext::RectExt,
    },
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
        let latitude = create_label!("Latitude:", parent, &rect);
        let rect = rect.to_next_line();
        let longitude = create_label!("Longitude:", parent, &rect);
        let rect = rect.to_next_line();
        let altitude = create_label!("Altitude:", parent, &rect);
        let rect = rect.to_next_line();
        let terrain = create_label!("Terrain:", parent, &rect);
        let block = Self {
            latitude,
            longitude,
            altitude,
            terrain,
        };
        Ok((block, rect))
    }

    pub fn update(
        &self,
        location: &LocationDataRef,
        terrain_probe: &TerrainProbe,
    ) -> ApiResult<()> {
        let coords = location.coords();
        let local = location.local();
        let terrain = terrain_probe.distance(local.x, local.y, local.z);
        update_widget(self.latitude, &format_f64(coords.latitude))?;
        update_widget(self.longitude, &format_f64(coords.longitude))?;
        update_widget(self.altitude, &format_altitude(coords.altitude))?;
        update_widget(self.terrain, &format_altitude(terrain))?;
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
