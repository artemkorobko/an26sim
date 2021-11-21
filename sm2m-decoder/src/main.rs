#![no_main]
#![no_std]

mod cdc;
mod led;
mod setup;

use cdc::{
    read::RequestType,
    write::{CdcWriter, Response},
};
use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
// use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::gpio;

use crate::{
    cdc::{
        device::CdcDevice,
        read::{CdcReader, Request, RequestTypeEx},
    },
    led::Led,
};

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // timer: u32,
        led: Led,
        usb_device: CdcDevice,
        usb_reader: CdcReader,
        usb_writer: CdcWriter,
    }

    // #[init(schedule = [blink])]
    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
        // setup::core(cx.core);
        let mut peripherals = setup::device(cx.device);

        // Configure output LED
        let led_pin = peripherals
            .gpioc
            .pc13
            .into_push_pull_output_with_state(&mut peripherals.gpioc.crh, gpio::State::Low);
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

        // Start blink task
        // let timer = peripherals.clocks.sysclk().0 / 2;
        // cx.schedule.blink(cx.start + timer.cycles()).unwrap();

        init::LateResources {
            // timer,
            led,
            usb_device,
            usb_reader: CdcReader::default(),
            usb_writer: CdcWriter::default(),
        }
    }

    #[idle()]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // asm::nop();
            asm::wfi();
        }
    }

    // #[task(resources = [timer, led], schedule = [blink])]
    // fn blink(cx: blink::Context) {
    //     cx.resources.led.toggle();
    //     cx.schedule
    //         .blink(cx.scheduled + cx.resources.timer.cycles())
    //         .ok();
    // }

    #[task(resources = [led, usb_device, usb_writer])]
    fn handle_request(cx: handle_request::Context, request: RequestType) {
        match request.parse() {
            Request::Ping(version, payload) => {
                let response = Response::Pong(version + 1, payload);
                let writer = cx.resources.usb_writer;
                writer.cache(response);
                writer.write(cx.resources.usb_device).ok();
            }
            _ => {
                // cx.resources.led.toggle();
                // flash LED
            }
        };

        cx.resources.led.toggle();
    }

    #[task(binds = USB_HP_CAN_TX, spawn = [handle_request], resources = [led, usb_device, usb_reader])]
    fn usb_tx(cx: usb_tx::Context) {
        read_request(cx.resources.usb_device, cx.resources.usb_reader).and_then(|request| {
            cx.resources.led.toggle();
            cx.spawn.handle_request(request).ok()
        });
    }

    #[task(binds = USB_LP_CAN_RX0, spawn = [handle_request], resources = [led, usb_device, usb_reader])]
    fn usb_rx0(cx: usb_rx0::Context) {
        read_request(cx.resources.usb_device, cx.resources.usb_reader).and_then(|request| {
            cx.resources.led.toggle();
            cx.spawn.handle_request(request).ok()
        });
    }

    extern "C" {
        fn TAMPER();
    }
};

fn read_request(device: &mut CdcDevice, reader: &mut CdcReader) -> Option<RequestType> {
    reader.read_from(device).unwrap_or(None)
}
