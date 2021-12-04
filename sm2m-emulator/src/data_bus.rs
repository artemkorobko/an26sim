use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::gpio::{gpioa, gpiob, Output, PushPull};

pub struct DataBus {
    pub interrupt: gpioa::PA0<Output<PushPull>>,
    pub data: gpiob::Parts,
}

impl DataBus {
    pub fn write(&mut self, value: u16) {
        self.interrupt.set_low().ok();
        self.interrupt.set_high().ok();
    }
}
