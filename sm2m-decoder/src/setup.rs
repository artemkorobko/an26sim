use stm32f1xx_hal::{device, gpio, prelude::*, rcc};

pub fn core(mut core: rtic::Peripherals) -> rtic::Peripherals {
    core.DWT.enable_cycle_counter();
    core
}

pub struct Peripherals {
    pub usb: device::USB,
    pub gpioa: gpio::gpioa::Parts,
    pub gpioc: gpio::gpioc::Parts,
    pub clocks: rcc::Clocks,
}

pub fn device(device: device::Peripherals) -> Peripherals {
    let mut flash = device.FLASH.constrain();
    let mut rcc = device.RCC.constrain();
    device.AFIO.constrain(&mut rcc.apb2);
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());
    Peripherals {
        usb: device.USB,
        gpioa: device.GPIOA.split(&mut rcc.apb2),
        gpioc: device.GPIOC.split(&mut rcc.apb2),
        clocks,
    }
}
