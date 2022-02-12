#![no_main]
#![no_std]

use panic_halt as _;

mod bus;
mod drivers;
mod params;
mod tasks;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [TAMP_STAMP])]
mod app {
    use stm32f4xx_hal::{gpio, otg_fs, prelude::*};

    use crate::bus;
    use crate::drivers::cdc_acm;
    use crate::params::{SM2MParamsState, MAX_PARAMS_COUNT};

    #[shared]
    struct Shared {
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        btn: gpio::gpioa::PA0<gpio::Input<gpio::PullDown>>,
        usb: cdc_acm::Device,
    }

    #[local]
    struct Local {
        state: SM2MParamsState,
        bus_interrupt: gpio::gpioa::PA9<gpio::Input<gpio::PullDown>>,
        bus: bus::DataBus,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let pac = cx.device;
        let clocks = pac
            .RCC
            .constrain()
            .cfgr
            .use_hse(25.mhz())
            .sysclk(84.mhz())
            .require_pll48clk()
            .freeze();

        // Configure LED
        let gpioc = pac.GPIOC.split();
        let led = gpioc
            .pc13
            .into_push_pull_output_in_state(gpio::PinState::High);

        // Configure button
        let gpioa = pac.GPIOA.split();
        let btn = gpioa.pa0.into_pull_down_input();

        // Configure USB
        let usb_conf = otg_fs::USB {
            usb_global: pac.OTG_FS_GLOBAL,
            usb_device: pac.OTG_FS_DEVICE,
            usb_pwrclk: pac.OTG_FS_PWRCLK,
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: gpioa.pa12.into_alternate(),
            hclk: clocks.hclk(),
        };

        let usb_descr = cdc_acm::Descriptor {
            vendor_id: 0x0483,
            product_id: 0x5740,
            manufacturer: "FSElectronics",
            product: "An26 SM2M Decoder",
            serial_number: todo!(),
        };

        let usb = cdc_acm::Device::new(usb_conf, usb_descr);

        // Configure data bus
        let gpiob = pac.GPIOB.split();
        let bus = bus::DataBus {
            bit0: gpioa.pa1,
            bit1: gpioa.pa2,
            bit2: gpioa.pa3,
            bit3: gpioa.pa4,
            bit4: gpioa.pa5,
            bit5: gpioa.pa6,
            bit6: gpioa.pa7,
            bit7: gpiob.pb0,
            bit8: gpiob.pb1,
            bit9: gpiob.pb2,
            bit10: gpiob.pb10,
            bit11: gpiob.pb12,
            bit12: gpiob.pb13,
            bit13: gpiob.pb14,
            bit14: gpiob.pb15,
            bit15: gpioa.pa8,
        };

        let mut syscfg = pac.SYSCFG.constrain();
        let mut bus_interrupt = gpioa.pa9.into_pull_down_input();
        bus_interrupt.make_interrupt_source(&mut syscfg);
        bus_interrupt.enable_interrupt(&mut pac.EXTI);
        bus_interrupt.trigger_on_edge(&mut pac.EXTI, gpio::Edge::Falling);
        bus_interrupt.clear_interrupt_pending_bit();

        (
            Shared { led, btn, usb },
            Local {
                state: SM2MParamsState::DetectMarker,
                bus_interrupt,
                bus,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    use crate::tasks::*;

    extern "Rust" {
        #[task(local = [state])]
        fn handle_param(cx: handle_param::Context, param: u16);
        #[task(shared = [usb])]
        fn transfer_params(cx: transfer_params::Context, params: [u16; MAX_PARAMS_COUNT]);
        #[task(priority = 2, binds = OTG_FS, shared = [usb])]
        fn usb_global(cx: usb_global::Context);
        #[task(priority = 2, binds = OTG_FS_WKUP, shared = [usb])]
        fn usb_wkup(cx: usb_wkup::Context);
        #[task(priority = 3, binds = EXTI9_5, local = [bus_interrupt, bus])]
        fn bus_read_interrupt(cx: bus_read_interrupt::Context);
    }
}
