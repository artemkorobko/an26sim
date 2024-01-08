use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{gpio, prelude::_embedded_hal_digital_ToggleableOutputPin};

type LedPin = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;

pub struct Led {
    pin: LedPin,
}

impl Led {
    pub fn new(pin: LedPin) -> Self {
        Self { pin }
    }

    pub fn toggle(&mut self) {
        self.pin.toggle().ok();
    }

    pub fn on(&mut self) {
        self.pin.set_low().ok();
    }

    pub fn off(&mut self) {
        self.pin.set_high().ok();
    }
}
