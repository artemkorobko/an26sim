#![no_main]
#![no_std]

mod cdc;
mod data_bus;
mod generators;

use data_bus::DataBus;
use embedded_hal::digital::v2::OutputPin;
use generators::Generators;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::{gpio, prelude::*, time, usb};

use cdc::{
    device::CdcDevice,
    inbound::{Reader, UsbInbound},
    outbound::{UsbOutbound, Writer},
};

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        sysclk: time::Hertz,
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        generators: Generators,
        usb_device: CdcDevice,
        data_bus: DataBus,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Configure MCU
        cx.core.DWT.enable_cycle_counter();
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();
        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        assert!(clocks.usbclk_valid());

        // Configure peripherals
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = cx.device.GPIOB.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, gpio::State::High);

        let (_, pb3, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);
        let data_bus = DataBus {
            interrupt: gpioa
                .pa0
                .into_push_pull_output_with_state(&mut gpioa.crl, gpio::State::High),
            bit0: gpiob
                .pb0
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit1: gpiob
                .pb1
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit2: gpiob
                .pb2
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit3: pb3.into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit4: pb4.into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit5: gpiob
                .pb5
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit6: gpiob
                .pb6
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit7: gpiob
                .pb7
                .into_push_pull_output_with_state(&mut gpiob.crl, gpio::State::High),
            bit8: gpiob
                .pb8
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit9: gpiob
                .pb9
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit10: gpiob
                .pb10
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit11: gpiob
                .pb11
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit12: gpiob
                .pb12
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit13: gpiob
                .pb13
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit14: gpiob
                .pb14
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
            bit15: gpiob
                .pb15
                .into_push_pull_output_with_state(&mut gpiob.crh, gpio::State::High),
        };

        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        usb_dp.set_low().ok();
        cortex_m::asm::delay(clocks.sysclk().0 / 100);
        let usb_conf = usb::Peripheral {
            usb: cx.device.USB,
            pin_dm: gpioa.pa11,
            pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
        };

        init::LateResources {
            sysclk: clocks.sysclk(),
            led,
            generators: Default::default(),
            usb_device: CdcDevice::new(usb_conf),
            data_bus,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(resources = [sysclk, led, generators, data_bus], schedule = [generate_params])]
    fn generate_params(cx: generate_params::Context) {
        let generators = cx.resources.generators;
        let bus = cx.resources.data_bus;
        if generators.enabled() {
            let gens = generators.inner_mut();
            for gen in gens {
                if let Some(generator) = gen {
                    let value = generator.generate();
                    bus.write(value);
                    bus.flush();
                    cortex_m::asm::delay(15); // 15us
                    bus.clear();
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

    #[task(resources = [generators, usb_device], schedule = [generate_params])]
    fn handle_usb_inbound(cx: handle_usb_inbound::Context, inbound: UsbInbound) {
        let mut usb_device = cx.resources.usb_device;
        let generators = cx.resources.generators;
        match inbound {
            UsbInbound::GetVersion => {
                let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
                let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
                let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u8>().unwrap_or(0);
                let outbound = UsbOutbound::Version(major, minor, patch);
                usb_device.lock(|device| {
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

    #[task(priority = 2, binds = USB_HP_CAN_TX, resources = [usb_device])]
    fn usb_tx(cx: usb_tx::Context) {
        cx.resources.usb_device.poll();
    }

    #[task(priority = 2, binds = USB_LP_CAN_RX0, spawn = [handle_usb_inbound], resources = [usb_device])]
    fn usb_rx0(cx: usb_rx0::Context) {
        let usb_device = cx.resources.usb_device;
        if usb_device.poll() {
            usb_device
                .read_ex()
                .unwrap_or(None)
                .and_then(|request| cx.spawn.handle_usb_inbound(request).ok());
        }
    }

    extern "C" {
        fn TAMPER();
    }
};
