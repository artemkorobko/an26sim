use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{
    gpio::{gpioa, gpiob, Output, PushPull},
    pac,
};

pub struct Interface {
    pub line_activity: u32,
    pub interrupt: gpioa::PA0<Output<PushPull>>,
    pub bit0: gpiob::PB0<Output<PushPull>>,
    pub bit1: gpiob::PB1<Output<PushPull>>,
    pub bit2: gpiob::PB2<Output<PushPull>>,
    pub bit3: gpiob::PB3<Output<PushPull>>,
    pub bit4: gpiob::PB4<Output<PushPull>>,
    pub bit5: gpiob::PB5<Output<PushPull>>,
    pub bit6: gpiob::PB6<Output<PushPull>>,
    pub bit7: gpiob::PB7<Output<PushPull>>,
    pub bit8: gpiob::PB8<Output<PushPull>>,
    pub bit9: gpiob::PB9<Output<PushPull>>,
    pub bit10: gpiob::PB10<Output<PushPull>>,
    pub bit11: gpiob::PB11<Output<PushPull>>,
    pub bit12: gpiob::PB12<Output<PushPull>>,
    pub bit13: gpiob::PB13<Output<PushPull>>,
    pub bit14: gpiob::PB14<Output<PushPull>>,
    pub bit15: gpiob::PB15<Output<PushPull>>,
}

impl Interface {
    pub fn write(&mut self, value: u16) {
        // UNSAFE: all pins of PORTB are set to output at this moment
        unsafe { (*pac::GPIOB::ptr()).odr.write(|w| w.bits(value as u32)) };
        self.interrupt.set_low().ok();
        cortex_m::asm::delay(self.line_activity);
        self.interrupt.set_high().ok();
    }
}
