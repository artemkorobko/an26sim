use rtic::Mutex;

use crate::{
    app::transfer_params,
    drivers::cdc_acm_outbound::{Outbound, Writer},
    params::MAX_PARAMS_COUNT,
};

pub fn transfer_params(
    mut cx: transfer_params::Context,
    params: [u16; MAX_PARAMS_COUNT],
    count: usize,
) {
    cx.shared
        .usb
        .lock(|device| device.write_outbound(Outbound::Params(params, count)))
        .ok();
}
