#![no_main]
#![no_std]

mod cdc;
mod led;
mod params;
mod params_generator;
mod setup;

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use params_generator::ParamsGenerator;
use stm32f1xx_hal::gpio;

use cdc::{
    device::CdcDevice,
    inbound::{PacketReader, UsbInPacket},
    outbound::{PacketWriter, UsbOutPacket},
};
use led::Led;
use params::SM2MParams;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
        params: SM2MParams,
        params_generator: ParamsGenerator,
        usb_device: CdcDevice,
    }

    #[init]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Setup MCU
        setup::core(&mut cx.core);
        let mut peripherals = setup::device(cx.device);

        // Configure output LED
        let led_pin = peripherals
            .gpioc
            .pc13
            .into_push_pull_output_with_state(&mut peripherals.gpioc.crh, gpio::State::High);
        let led = Led::new(led_pin);

        // Configure USB CDC device
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
        let usb_dp = usb_dp.into_floating_input(&mut peripherals.gpioa.crh);
        let usb_dm = peripherals.gpioa.pa11;
        let usb_device = CdcDevice::new(peripherals.usb, usb_dm, usb_dp);

        init::LateResources {
            led,
            params: Default::default(),
            params_generator: Default::default(),
            usb_device,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task]
    fn generate_param(cx: generate_param::Context) {}

    #[task(capacity = 5, resources = [led, params, params_generator, usb_device], schedule = [generate_param])]
    fn handle_usb_inbound(cx: handle_usb_inbound::Context, inbound: UsbInPacket) {
        let mut usb_device = cx.resources.usb_device;
        match inbound {
            UsbInPacket::GetVersion => {
                let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap_or(0);
                let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap_or(0);
                let outbound = UsbOutPacket::Version(major, minor);
                usb_device.lock(|usb_device| {
                    write_usb_packet(usb_device, outbound);
                });
            }
            UsbInPacket::Ping(version, payload) => {
                let response = UsbOutPacket::Pong(version.wrapping_add(1), payload);
                usb_device.lock(|usb_device| {
                    write_usb_packet(usb_device, response);
                });
            }
            UsbInPacket::LedOn => cx.resources.led.on(),
            UsbInPacket::LedOff => cx.resources.led.off(),
            UsbInPacket::SetParam(index, param) => {
                cx.resources.params.set(index as usize, param);
            }
            UsbInPacket::GetParam(index) => {
                if let Some(param) = cx.resources.params.get(index as usize) {
                    let response = UsbOutPacket::Param(index, param);
                    usb_device.lock(|usb_device| write_usb_packet(usb_device, response))
                }
            }
            UsbInPacket::EnableParamGenerator(index, period, step) => {
                cx.resources
                    .params_generator
                    .enable(index as usize, period, step);
            }
            UsbInPacket::DisableParamGenerator(index) => {
                cx.resources.params_generator.disable(index as usize);
            }
            UsbInPacket::EnableGlobalParamGenerator(freq) => {
                cx.schedule.generate_param(cx.scheduled).ok();
            }
            UsbInPacket::DisableGlobalParamGenerator => {}
        };
    }

    #[task(priority = 2, binds = USB_HP_CAN_TX, resources = [usb_device])]
    fn usb_tx(cx: usb_tx::Context) {
        cx.resources.usb_device.poll();
    }

    #[task(priority = 2, binds = USB_LP_CAN_RX0, spawn = [handle_usb_inbound], resources = [usb_device])]
    fn usb_rx0(cx: usb_rx0::Context) {
        if cx.resources.usb_device.poll() {
            read_usb_packet(cx.resources.usb_device)
                .and_then(|request| cx.spawn.handle_usb_inbound(request).ok());
        }
    }

    extern "C" {
        fn TAMPER();
    }
};

fn read_usb_packet(device: &mut CdcDevice) -> Option<UsbInPacket> {
    device.read_packet().unwrap_or(None)
}

fn write_usb_packet(device: &mut CdcDevice, packet: UsbOutPacket) {
    device.write_packet(packet).ok();
}
