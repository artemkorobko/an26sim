#![no_main]
#![no_std]

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::{gpio, pac, prelude::*};

const SYS_FREQ: u32 = 72_000_000;
const TIMER_FREQ: u32 = SYS_FREQ / 20;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        let mut core: rtic::Peripherals = cx.core;
        core.DWT.enable_cycle_counter();

        let device: pac::Peripherals = cx.device;
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut _afio = device.AFIO.constrain(&mut rcc.apb2);
        rcc.cfgr
            .use_hse(8.mhz())
            .sysclk(SYS_FREQ.hz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, gpio::State::Low);
        led.set_low().unwrap();

        cx.schedule.blink(cx.start + TIMER_FREQ.cycles()).unwrap();

        init::LateResources { led }
    }

    #[idle()]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    #[task(resources = [led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle().unwrap();
        cx.schedule
            .blink(cx.scheduled + TIMER_FREQ.cycles())
            .unwrap();
    }

    extern "C" {
        fn TAMPER();
    }
};
