use rtic::Mutex;

use crate::{
    app::usb_rx,
    drivers::{
        cdc_acm::Device,
        cdc_acm_inbound::{Inbound, Reader},
        cdc_acm_outbound::{Outbound, Writer},
    },
};

pub fn usb_rx(mut cx: usb_rx::Context) {
    if let Some(inbound) = cx.shared.usb.lock(poll) {
        if let Some(outbound) = handle_inbound(&mut cx, inbound) {
            send(&mut cx, outbound);
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

fn send(cx: &mut usb_rx::Context, outbound: Outbound) {
    cx.shared.usb.lock(|device| {
        device.write_outbound(outbound).ok();
    })
}

fn handle_inbound(cx: &mut usb_rx::Context, inbound: Inbound) -> Option<Outbound> {
    match inbound {
        Inbound::FirmwareVersion => firmware_version(),
        Inbound::EnableGenerator(idx, val, period, step) => None,
        Inbound::DisableGenerator(idx) => None,
        Inbound::StartTimer(fps) => None,
        Inbound::StopTimer => None,
        Inbound::Unknown => None,
    }
}

fn firmware_version() -> Option<Outbound> {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
    Some(Outbound::Version(major, minor, patch))
}
