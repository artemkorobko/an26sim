use std::time::Duration;

use xplm::geometry::Rect;
use xplm_sys::XPWidgetID;

use crate::{
    create_label,
    io::metrics::{IOMetrics, IOState},
    xplane::inspector::rect_ext::RectExt,
};

use super::api::{update_widget, ApiResult};

pub struct IOBlock {
    in_state: XPWidgetID,
    in_transferred: XPWidgetID,
    in_packets: XPWidgetID,
    in_speed: XPWidgetID,
    in_errors: XPWidgetID,
    out_state: XPWidgetID,
    out_transferred: XPWidgetID,
    out_packets: XPWidgetID,
    out_speed: XPWidgetID,
    out_errors: XPWidgetID,
}

impl IOBlock {
    pub fn new(parent: XPWidgetID, rect: &Rect<i32>) -> ApiResult<(Self, Rect<i32>)> {
        let in_state = create_label!("\u{21e2} State:", parent, rect);
        let rect = rect.to_next_line();
        let in_transferred = create_label!("\u{21e2} Received:", parent, &rect);
        let rect = rect.to_next_line();
        let in_packets = create_label!("\u{21e2} Packets:", parent, &rect);
        let rect = rect.to_next_line();
        let in_speed = create_label!("\u{21e2} Speed:", parent, &rect);
        let rect = rect.to_next_line();
        let in_errors = create_label!("\u{21e2} Errors:", parent, &rect);
        let rect = rect.to_next_block();
        let out_state = create_label!("\u{21e0} State:", parent, &rect);
        let rect = rect.to_next_line();
        let out_transferred = create_label!("\u{21e0} Sent:", parent, &rect);
        let rect = rect.to_next_line();
        let out_packets = create_label!("\u{21e0} Packets:", parent, &rect);
        let rect = rect.to_next_line();
        let out_speed = create_label!("\u{21e0} Speed:", parent, &rect);
        let rect = rect.to_next_line();
        let out_errors = create_label!("\u{21e0} Errors:", parent, &rect);
        Ok((
            Self {
                in_state,
                in_transferred,
                in_packets,
                in_speed,
                in_errors,
                out_state,
                out_transferred,
                out_packets,
                out_speed,
                out_errors,
            },
            rect,
        ))
    }

    pub fn update(
        &self,
        input: &mut IOMetrics,
        output: &mut IOMetrics,
        delta: &Duration,
    ) -> ApiResult<()> {
        update_widget(self.in_state, format_state(&input.state))?;
        update_widget(self.in_transferred, &format_size(input.transferred))?;
        update_widget(self.in_packets, &format!("{}", input.packets))?;
        update_widget(self.in_speed, &format_speed(input.bps(delta)))?;
        update_widget(self.in_errors, &format!("{}", input.errors))?;
        update_widget(self.out_state, format_state(&output.state))?;
        update_widget(self.out_transferred, &format_size(output.transferred))?;
        update_widget(self.out_packets, &format!("{}", output.packets))?;
        update_widget(self.out_speed, &format_speed(output.bps(delta)))?;
        update_widget(self.out_errors, &format!("{}", output.errors))?;
        Ok(())
    }
}

fn format_state(state: &IOState) -> &'static str {
    match state {
        IOState::Connected => "connected",
        IOState::Disconnected => "disconnected",
    }
}

fn format_size(value: usize) -> String {
    if value < 1000 {
        format!("{}B", value)
    } else if value < (1000 * 1024) {
        format!("{:.2}Kb", value as f64 / 1024.0)
    } else {
        format!("{:.2}Mb", value as f64 / 1048576.0)
    }
}

fn format_speed(value: f32) -> String {
    if value < 1000.0 {
        format!("{:.2}Bps", value)
    } else {
        format!("{:.2}Kbps", value / 1024.0)
    }
}
