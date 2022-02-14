#![no_main]
#![no_std]

mod bus;
mod device_id;
mod drivers;
mod generators;
mod tasks;

use panic_halt as _;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [TAMPER])]
mod app {
    use stm32f1xx_hal::{gpio, prelude::*, time, usb};

    use crate::{bus, device_id, drivers::cdc_acm, generators::Generators};

    #[shared]
    struct Shared {
        usb: cdc_acm::Device,
    }

    #[local]
    struct Local {}

    // struct Resources {
    //     sysclk: time::Hertz,
    //     led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
    //     generators: Generators,
    //     usb: Device,
    //     bus: bus::Interface,
    // }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let mut pac = cx.device;
        let mut flash = pac.FLASH.constrain();
        let mut rcc = pac.RCC.constrain();
        let mut afio = pac.AFIO.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        // Disable JTAG
        let mut gpioa = pac.GPIOA.split();
        let mut gpiob = pac.GPIOB.split();
        let (_, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure LED
        let mut gpioc = pac.GPIOC.split();
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, gpio::PinState::Low);

        // Configure USB
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let usb_dp = gpioa
            .pa12
            .into_push_pull_output_with_state(&mut gpioa.crh, gpio::PinState::Low);
        let cpu_cycles_hz = clocks.sysclk().0;
        cortex_m::asm::delay(cpu_cycles_hz / 100);
        let usb_peripheral = usb::Peripheral {
            usb: pac.USB,
            pin_dm: gpioa.pa11,
            pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
        };
        let usb_descriptor = cdc_acm::Descriptor {
            vendor_id: 0x0483,
            product_id: 0x5740,
            manufacturer: "FSElectronics",
            product: "SM2M Emulator",
            serial_number: device_id::read_str(),
        };
        let usb = cdc_acm::Device::new(usb_peripheral, usb_descriptor);

        // Configure data bus
        let line_activity = cpu_cycles_hz; // 1 sec
        let bus = bus::Interface {
            line_activity,
            interrupt: gpioa
                .pa0
                .into_push_pull_output_with_state(&mut gpioa.crl, gpio::PinState::Low),
            bit0: gpiob
                .pb0
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit1: gpiob
                .pb1
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit2: gpiob
                .pb2
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit3: pb3.into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit4: pb4.into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit5: gpiob
                .pb5
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit6: gpiob
                .pb6
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit7: gpiob
                .pb7
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::PinState::Low),
            bit8: gpiob
                .pb8
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit9: gpiob
                .pb9
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit10: gpiob
                .pb10
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit11: gpiob
                .pb11
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit12: gpiob
                .pb12
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit13: gpiob
                .pb13
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit14: gpiob
                .pb14
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
            bit15: gpiob
                .pb15
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::PinState::Low),
        };

        (Shared { usb }, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    use crate::tasks::*;

    extern "Rust" {
        #[task(binds = USB_HP_CAN_TX, shared = [usb])]
        fn usb_tx(cx: usb_tx::Context);
        #[task(binds = USB_LP_CAN_RX0, shared = [usb])]
        fn usb_rx(cx: usb_rx::Context);
    }

    // #[task(resources = [sysclk, led, generators, bus], schedule = [generate_params])]
    // fn generate_params(cx: generate_params::Context) {
    //     let generators = cx.resources.generators;
    //     let bus = cx.resources.bus;
    //     if generators.enabled() {
    //         let gens = generators.inner_mut();
    //         for gen in gens {
    //             if let Some(generator) = gen {
    //                 let value = generator.generate();
    //                 bus.write(value);
    //             }
    //         }

    //         cx.resources.led.toggle().ok();
    //         let delay = (cx.resources.sysclk.0 / generators.fps() as u32).cycles();
    //         let schedule = cx.scheduled + delay;
    //         cx.schedule.generate_params(schedule).ok();
    //     } else {
    //         cx.resources.led.set_high().ok();
    //     }
    // }

    // #[task(resources = [generators, usb], schedule = [generate_params])]
    // fn handle_usb_inbound(cx: handle_usb_inbound::Context, inbound: Inbound) {
    //     let mut usb = cx.resources.usb;
    //     let generators = cx.resources.generators;
    //     match inbound {
    //         Inbound::EnableGenerator(index, period, value, step) => {
    //             generators.enable_generator(index as usize, value, period, step);
    //         }
    //         Inbound::DisableGenerator(index) => {
    //             generators.disable_generator(index as usize);
    //         }
    //         Inbound::StartProducer(fps) => {
    //             if generators.enable(fps) {
    //                 cx.schedule.generate_params(cx.scheduled).ok();
    //             }
    //         }
    //         Inbound::StopProducer => {
    //             generators.disable();
    //         }
    //     };
    // }
}
