use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::gpio::{gpioa, gpiob, Output, PushPull};

macro_rules! set_state {
    ($value:expr, $bit:expr, $pin:expr) => {{
        let is_bit_set = $value >> $bit & 1 == 1;
        if is_bit_set {
            $pin.set_low().ok();
        } else {
            $pin.set_high().ok();
        }
    }};
}

pub struct DataBus {
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

impl DataBus {
    pub fn write(&mut self, value: u16) {
        set_state!(value, 0, self.bit0);
        set_state!(value, 1, self.bit1);
        set_state!(value, 2, self.bit2);
        set_state!(value, 3, self.bit3);
        set_state!(value, 4, self.bit4);
        set_state!(value, 5, self.bit5);
        set_state!(value, 6, self.bit6);
        set_state!(value, 7, self.bit7);
        set_state!(value, 8, self.bit8);
        set_state!(value, 9, self.bit9);
        set_state!(value, 10, self.bit10);
        set_state!(value, 11, self.bit11);
        set_state!(value, 12, self.bit12);
        set_state!(value, 13, self.bit13);
        set_state!(value, 14, self.bit14);
        set_state!(value, 15, self.bit15);
    }

    pub fn flush(&mut self) {
        self.interrupt.set_low().ok();
    }

    pub fn clear(&mut self) {
        self.interrupt.set_high().ok();
    }
}
