#![no_main]
#![no_std]

mod cdc;
mod generators;
mod setup;
mod signals;

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use generators::Generators;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::{
    gpio::{self, gpioc::PC13, Output, PushPull},
    prelude::_embedded_hal_digital_ToggleableOutputPin,
    time, usb,
};

use cdc::{
    device::CdcDevice,
    inbound::{Reader, UsbInbound},
    outbound::{UsbOutbound, Writer},
};
use signals::SignalsWriter;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        sysclk: time::Hertz,
        led: PC13<Output<PushPull>>,
        signals: SignalsWriter,
        generators: Generators,
        usb_device: CdcDevice,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Setup MCU
        setup::core(&mut cx.core);
        let mut peripherals = setup::device(cx.device);

        // Configure output LED
        let led = peripherals
            .gpioc
            .pc13
            .into_push_pull_output_with_state(&mut peripherals.gpioc.crh, gpio::State::High);
        let signals = SignalsWriter::new();

        // Configure USB
        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = peripherals
            .gpioa
            .pa12
            .into_push_pull_output(&mut peripherals.gpioa.crh);
        usb_dp.set_low().ok();
        asm::delay(peripherals.clocks.sysclk().0 / 100);
        let usb_conf = usb::Peripheral {
            usb: peripherals.usb,
            pin_dm: peripherals.gpioa.pa11,
            pin_dp: usb_dp.into_floating_input(&mut peripherals.gpioa.crh),
        };

        init::LateResources {
            sysclk: peripherals.clocks.sysclk(),
            led,
            signals,
            generators: Default::default(),
            usb_device: CdcDevice::new(usb_conf),
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(resources = [sysclk, led, generators, usb_device], schedule = [generate_params])]
    fn generate_params(cx: generate_params::Context, marker: u16) {
        let generators = cx.resources.generators;
        if generators.enabled() {
            cx.resources.led.toggle().ok();
            let schedule = (cx.resources.sysclk.0 / generators.fps() as u32).cycles();
            cx.schedule
                .generate_params(cx.scheduled + schedule, marker)
                .ok();
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
            UsbInbound::UpdateParam(index, value) => {
                generators.update_param(index as usize, value);
            }
            UsbInbound::EnableGenerator(index, period, step) => {
                generators.enable_generator(index as usize, period, step);
            }
            UsbInbound::DisableGenerator(index) => {
                generators.disable_generator(index as usize);
            }
            UsbInbound::StartGenerator(fps, marker) => {
                generators.enable(fps);
                cx.schedule.generate_params(cx.scheduled, marker).ok();
            }
            UsbInbound::StopGenerator => {
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
