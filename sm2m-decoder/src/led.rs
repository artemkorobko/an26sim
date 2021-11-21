use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{gpio, prelude::_embedded_hal_digital_ToggleableOutputPin};

pub struct Led {
    pin: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
}

impl Led {
    pub fn new(mut pin: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>) -> Self {
        pin.set_low().ok();
        Self { pin }
    }

    pub fn toggle(&mut self) {
        self.pin.toggle().ok();
    }

    pub fn on(&mut self) {
        self.pin.set_high().ok();
    }

    pub fn off(&mut self) {
        self.pin.set_low().ok();
    }
}
