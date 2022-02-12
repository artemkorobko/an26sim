use stm32f4xx_hal::{
    gpio::{gpioa, gpiob, Floating, Input},
    pac,
};

pub struct DataBus {
    pub bit0: gpioa::PA1<Input<Floating>>,
    pub bit1: gpioa::PA2<Input<Floating>>,
    pub bit2: gpioa::PA3<Input<Floating>>,
    pub bit3: gpioa::PA4<Input<Floating>>,
    pub bit4: gpioa::PA5<Input<Floating>>,
    pub bit5: gpioa::PA6<Input<Floating>>,
    pub bit6: gpioa::PA7<Input<Floating>>,
    pub bit7: gpiob::PB0<Input<Floating>>,
    pub bit8: gpiob::PB1<Input<Floating>>,
    pub bit9: gpiob::PB2<Input<Floating>>,
    pub bit10: gpiob::PB10<Input<Floating>>,
    pub bit11: gpiob::PB12<Input<Floating>>,
    pub bit12: gpiob::PB13<Input<Floating>>,
    pub bit13: gpiob::PB14<Input<Floating>>,
    pub bit14: gpiob::PB15<Input<Floating>>,
    pub bit15: gpioa::PA8<Input<Floating>>,
}

impl DataBus {
    pub fn read(&self) -> u16 {
        let gpioa_bits = unsafe { (*pac::GPIOA::ptr()).idr.read().bits() as u16 };
        let gpiob_bits = unsafe { (*pac::GPIOB::ptr()).idr.read().bits() as u16 };
        merge_bits(gpioa_bits, gpiob_bits)
    }
}

fn merge_bits(mut gpioa_bits: u16, gpiob_bits: u16) -> u16 {
    gpioa_bits |= (gpiob_bits & 0x7) << 7; // merge pb0, pb1 and pb2 into bits 7, 8 and 9
    gpioa_bits |= gpiob_bits & 0x400; // merge pb10 into bit 10
    gpioa_bits |= (gpiob_bits & 0xF000) >> 1; // merge pb12, pb13, pb14 and pb15 into bit 11, 12, 13 and 14
    gpioa_bits
}
