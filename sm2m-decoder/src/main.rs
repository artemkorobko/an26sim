#![no_main]
#![no_std]

mod led;
mod setup;
mod usb;

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::gpio;

use crate::{
    led::Led,
    usb::{cdc_device::CDCDevice, command::USBCommand, reader::USBReader},
};

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        timer: u32,
        led: Led,
        usb_device: CDCDevice,
        usb_reader: USBReader,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
        setup::core(cx.core);
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
        usb_dp.set_low().unwrap();
        asm::delay(peripherals.clocks.sysclk().0 / 100);
        let usb_dp = usb_dp.into_floating_input(&mut peripherals.gpioa.crh);
        let usb_dm = peripherals.gpioa.pa11;
        let usb_device = CDCDevice::new(peripherals.usb, usb_dm, usb_dp);
        let usb_reader = USBReader::default();

        // Start blink task
        let timer = peripherals.clocks.sysclk().0 / 20;
        cx.schedule.blink(cx.start + timer.cycles()).unwrap();

        init::LateResources {
            timer,
            led,
            usb_device,
            usb_reader,
        }
    }

    #[idle()]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            asm::nop();
        }
    }

    #[task(resources = [timer, led], schedule = [blink])]
    fn blink(cx: blink::Context) {
        cx.resources.led.toggle();
        cx.schedule
            .blink(cx.scheduled + cx.resources.timer.cycles())
            .unwrap();
    }

    #[task(binds = USB_HP_CAN_TX, resources = [usb_device, usb_reader])]
    fn usb_tx(cx: usb_tx::Context) {
        handle_usb_command(cx.resources.usb_device, cx.resources.usb_reader);
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_device, usb_reader])]
    fn usb_rx0(cx: usb_rx0::Context) {
        handle_usb_command(cx.resources.usb_device, cx.resources.usb_reader);
    }

    extern "C" {
        fn TAMPER();
    }
};

fn handle_usb_command(usb_device: &mut CDCDevice, usb_reader: &mut USBReader) {
    if let Some(command) = usb_reader.read_command(usb_device) {
        match command {
            USBCommand::Ping => usb_device.write(&[USBCommand::Pong.to_u8()]).ok(),
            _ => None,
        };
    }
}
