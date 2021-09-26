#![no_main]
#![no_std]

use panic_halt as _;
use rtic::cyccnt;
use stm32f1xx_hal::{gpio, pac, prelude::*};

#[rtic::app(device = stm32f1xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut core: rtic::Peripherals = cx.core;
        let device: pac::Peripherals = cx.device;
        let mut rcc = device.RCC.constrain();
        rcc.cfgr.use_hse(8.mhz()).sysclk(72.mhz());
        core.DCB.enable_trace(); // Enable the monotonic timer (CYCCNT)

        cx.schedule
            .blink(cx.start + cyccnt::Duration::from_cycles(72.mhz().0))
            .unwrap();

        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        init::LateResources { led }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle().unwrap();
        // cx.schedule
        //     .blink(cx.scheduled + cyccnt::Duration::from_cycles(72.mhz().0 / 2))
        //     .unwrap();
    }

    extern "C" {
        fn TAMPER();
    }
};
