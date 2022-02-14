use rtic::Mutex;

use crate::{app::usb_tx, drivers::cdc_acm::Device};

pub fn usb_tx(mut cx: usb_tx::Context) {
    cx.shared.usb.lock(Device::poll);
}
