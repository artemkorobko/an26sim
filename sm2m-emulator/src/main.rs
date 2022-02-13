#![no_main]
#![no_std]

mod bus;
mod drivers;
mod generators;

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::{gpio, prelude::*, time, usb};

use drivers::prelude::*;
use generators::Generators;

macro_rules! as_output {
    ($pin:expr, $cr:expr) => {{
        $pin.into_push_pull_output_with_state($cr, gpio::State::High)
    }};
}

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        sysclk: time::Hertz,
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        generators: Generators,
        usb: Device,
        bus: bus::Interface,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
        let mut cp = cx.core;
        cp.DWT.enable_cycle_counter();

        // Configure peripherals
        let mut pac = cx.device;
        let mut flash = pac.FLASH.constrain();
        let mut rcc = pac.RCC.constrain();
        let mut afio = pac.AFIO.constrain(&mut rcc.apb2);
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        // Disable JTAG
        let mut gpioa = pac.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = pac.GPIOB.split(&mut rcc.apb2);
        let (_, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        // Configure LED
        let mut gpioc = pac.GPIOC.split(&mut rcc.apb2);
        let led = as_output!(gpioc.pc13, &mut gpioc.crh);

        // Configure USB
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = as_output!(gpioa.pa12, &mut gpioa.crh);
        usb_dp.set_low().ok();
        let cpu_cycles_hz = clocks.sysclk().0;
        cortex_m::asm::delay(cpu_cycles_hz / 100);
        let usb_conf = usb::Peripheral {
            usb: pac.USB,
            pin_dm: gpioa.pa11,
            pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
        };

        let usb = Device::new(usb_conf);

        // Configure data bus
        let line_activity = cpu_cycles_hz; // 1 sec
        let bus = bus::Interface {
            line_activity,
            interrupt: as_output!(gpioa.pa0, &mut gpioa.crl),
            bit0: as_output!(gpiob.pb0, &mut gpiob.crl),
            bit1: as_output!(gpiob.pb1, &mut gpiob.crl),
            bit2: as_output!(gpiob.pb2, &mut gpiob.crl),
            bit3: as_output!(pb3, &mut gpiob.crl),
            bit4: as_output!(pb4, &mut gpiob.crl),
            bit5: as_output!(gpiob.pb5, &mut gpiob.crl),
            bit6: as_output!(gpiob.pb6, &mut gpiob.crl),
            bit7: as_output!(gpiob.pb7, &mut gpiob.crl),
            bit8: as_output!(gpiob.pb8, &mut gpiob.crh),
            bit9: as_output!(gpiob.pb9, &mut gpiob.crh),
            bit10: as_output!(gpiob.pb10, &mut gpiob.crh),
            bit11: as_output!(gpiob.pb11, &mut gpiob.crh),
            bit12: as_output!(gpiob.pb12, &mut gpiob.crh),
            bit13: as_output!(gpiob.pb13, &mut gpiob.crh),
            bit14: as_output!(gpiob.pb14, &mut gpiob.crh),
            bit15: as_output!(gpiob.pb15, &mut gpiob.crh),
        };

        init::LateResources {
            sysclk: clocks.sysclk(),
            led,
            generators: Default::default(),
            usb,
            bus,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(resources = [sysclk, led, generators, bus], schedule = [generate_params])]
    fn generate_params(cx: generate_params::Context) {
        let generators = cx.resources.generators;
        let bus = cx.resources.bus;
        if generators.enabled() {
            let gens = generators.inner_mut();
            for gen in gens {
                if let Some(generator) = gen {
                    let value = generator.generate();
                    bus.write(value);
                }
            }

            cx.resources.led.toggle().ok();
            let delay = (cx.resources.sysclk.0 / generators.fps() as u32).cycles();
            let schedule = cx.scheduled + delay;
            cx.schedule.generate_params(schedule).ok();
        } else {
            cx.resources.led.set_high().ok();
        }
    }

    #[task(resources = [generators, usb], schedule = [generate_params])]
    fn handle_usb_inbound(cx: handle_usb_inbound::Context, inbound: UsbInbound) {
        let mut usb = cx.resources.usb;
        let generators = cx.resources.generators;
        match inbound {
            UsbInbound::GetVersion => {
                let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
                let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
                let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
                let outbound = UsbOutbound::Version(major, minor, patch);
                usb.lock(|device| {
                    device.write_ex(outbound).ok();
                });
            }
            UsbInbound::EnableGenerator(index, period, value, step) => {
                generators.enable_generator(index as usize, value, period, step);
            }
            UsbInbound::DisableGenerator(index) => {
                generators.disable_generator(index as usize);
            }
            UsbInbound::StartProducer(fps) => {
                if generators.enable(fps) {
                    cx.schedule.generate_params(cx.scheduled).ok();
                }
            }
            UsbInbound::StopProducer => {
                generators.disable();
            }
        };
    }

    #[task(priority = 2, binds = USB_HP_CAN_TX, resources = [usb])]
    fn usb_tx(cx: usb_tx::Context) {
        cx.resources.usb.poll();
    }

    #[task(priority = 2, binds = USB_LP_CAN_RX0, spawn = [handle_usb_inbound], resources = [usb])]
    fn usb_rx0(cx: usb_rx0::Context) {
        let usb = cx.resources.usb;
        if usb.poll() {
            usb.read_ex()
                .unwrap_or(None)
                .and_then(|request| cx.spawn.handle_usb_inbound(request).ok());
        }
    }

    extern "C" {
        fn TAMPER();
    }
};
