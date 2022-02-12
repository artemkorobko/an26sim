use stm32f4xx_hal::gpio::ExtiPin;

use crate::app::{bus_read_interrupt, handle_param};

pub fn bus_read_interrupt(cx: bus_read_interrupt::Context) {
    let pin = cx.local.bus_interrupt;
    if pin.check_interrupt() {
        let param = cx.local.bus.read();
        handle_param::spawn(param).ok();
        pin.clear_interrupt_pending_bit();
    }
}
