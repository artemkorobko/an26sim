use stm32f4xx_hal::{
    gpio::{gpioa, gpioc},
    pac,
    prelude::*,
    rcc,
};

pub struct Peripherals {
    pub usb_global: pac::OTG_FS_GLOBAL,
    pub usb_device: pac::OTG_FS_DEVICE,
    pub usb_pwrclk: pac::OTG_FS_PWRCLK,
    pub gpioa: gpioa::Parts,
    pub gpioc: gpioc::Parts,
    pub clocks: rcc::Clocks,
}

pub fn core(cp: &mut cortex_m::Peripherals) {
    cp.DWT.enable_cycle_counter();
}

pub fn device(dp: pac::Peripherals) -> Peripherals {
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(25.mhz())
        .sysclk(84.mhz())
        .require_pll48clk()
        .freeze();

    Peripherals {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        gpioa: dp.GPIOA.split(),
        gpioc: dp.GPIOC.split(),
        clocks,
    }
}
