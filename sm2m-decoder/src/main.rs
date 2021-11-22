#![no_main]
#![no_std]

mod cdc;
mod led;
mod params;
mod setup;

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use stm32f1xx_hal::gpio;

// use cdc::* as cdc;
use cdc::{
    device::CdcDevice,
    request::{Request, RequestReader},
    response::{Response, ResponseWriter},
};
use led::Led;
use params::SM2MParams;

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Led,
        params: SM2MParams,
        usb_device: CdcDevice,
        usb_reader: RequestReader,
        usb_writer: ResponseWriter,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
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
            usb_device,
            usb_reader: Default::default(),
            usb_writer: Default::default(),
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            asm::wfi();
        }
    }

    #[task(capacity = 5, resources = [led, usb_device, usb_writer])]
    fn handle_request(cx: handle_request::Context, request: Request) {
        match request {
            Request::Ping(version, payload) => {
                let response = Response::Pong(version.wrapping_add(1), payload);
                let mut usb_device = cx.resources.usb_device;
                let usb_writer = cx.resources.usb_writer;
                usb_device.lock(|usb_device| {
                    write_usb_response(usb_device, usb_writer, response);
                });
            }
            Request::LedOn => cx.resources.led.on(),
            Request::LedOff => cx.resources.led.off(),
            _ => {}
        };
    }

    #[task(priority = 2, binds = USB_HP_CAN_TX, resources = [usb_device])]
    fn usb_tx(cx: usb_tx::Context) {
        cx.resources.usb_device.poll();
    }

    #[task(priority = 2, binds = USB_LP_CAN_RX0, spawn = [handle_request], resources = [usb_device, usb_reader])]
    fn usb_rx0(cx: usb_rx0::Context) {
        if cx.resources.usb_device.poll() {
            read_usb_request(cx.resources.usb_device, cx.resources.usb_reader)
                .and_then(|request| cx.spawn.handle_request(request).ok());
        }
    }

    extern "C" {
        fn TAMPER();
    }
};

fn read_usb_request(device: &mut CdcDevice, reader: &mut RequestReader) -> Option<Request> {
    reader.read_from(device).unwrap_or(None)
}

fn write_usb_response(device: &mut CdcDevice, writer: &mut ResponseWriter, response: Response) {
    writer.write(device, response).ok();
}
