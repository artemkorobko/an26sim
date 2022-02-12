use rtic::Mutex;

use crate::{
    app::{usb_global, usb_wkup},
    drivers::{
        cdc_acm::Device,
        cdc_acm_inbound::{Inbound, Reader},
        cdc_acm_outbound::{Outbound, Writer},
    },
};

pub fn usb_global(mut cx: usb_global::Context) {
    if let Some(inbound) = cx.shared.usb.lock(poll) {
        if let Some(outbound) = handle_inbound(inbound) {
            cx.shared
                .usb
                .lock(|device| device.write_outbound(outbound))
                .ok();
        }
    }
}

pub fn usb_wkup(mut cx: usb_wkup::Context) {
    if let Some(inbound) = cx.shared.usb.lock(poll) {
        if let Some(outbound) = handle_inbound(inbound) {
            cx.shared
                .usb
                .lock(|device| device.write_outbound(outbound))
                .ok();
        }
    }
}

fn poll(device: &mut Device) -> Option<Inbound> {
    if device.poll() {
        device.read_inbound().ok()
    } else {
        None
    }
}

fn handle_inbound(inbound: Inbound) -> Option<Outbound> {
    match inbound {
        Inbound::FirmwareVersion => {
            let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
            let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
            let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
            Some(Outbound::FirmwareVersion(major, minor, patch))
        }
        Inbound::Unknown => None,
    }
}
