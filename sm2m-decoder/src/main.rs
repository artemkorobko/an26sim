#![no_main]
#![no_std]

use core::borrow::BorrowMut;

use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::{gpio, pac, prelude::*, usb};
use usb_device::{
    class_prelude::{UsbBus, UsbBusAllocator},
    prelude::*,
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[rtic::app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        timer: u32,
        led: gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>,
        usb_dev: UsbDevice<'static, usb::UsbBusType>,
        serial: SerialPort<'static, usb::UsbBusType>,
    }

    #[init(schedule = [blink])]
    fn init(cx: init::Context) -> init::LateResources {
        // Setup MCU
        let mut core: rtic::Peripherals = cx.core;
        core.DWT.enable_cycle_counter();

        let device: pac::Peripherals = cx.device;
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut _afio = device.AFIO.constrain(&mut rcc.apb2);
        let clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // Configure output LED
        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, gpio::State::Low);
        led.set_low().unwrap();

        // Configure USB CDC device
        assert!(clocks.usbclk_valid());
        static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBusType>> = None;

        // BluePill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
        usb_dp.set_low().unwrap();
        asm::delay(clocks.sysclk().0 / 100);
        let usb_dm = gpioa.pa11;
        let usb_dp = usb_dp.into_floating_input(&mut gpioa.crh);
        let usb = usb::Peripheral {
            usb: device.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        let (usb_dev, serial) = unsafe {
            *USB_BUS.borrow_mut() = Some(usb::UsbBus::new(usb));
            let serial = SerialPort::new(USB_BUS.as_ref().unwrap());
            let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(1155, 22336))
                .manufacturer("STMicroelectronics")
                .product("STM32 Virtual ComPort")
                .serial_number("SM2M-DECODER")
                .device_class(USB_CLASS_CDC)
                .build();
            (usb_dev, serial)
        };

        cx.schedule
            .blink(cx.start + clocks.sysclk().0.cycles())
            .unwrap();

        let timer = clocks.sysclk().0 / 20;
        init::LateResources {
            timer,
            led,
            usb_dev,
            serial,
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
        cx.resources.led.toggle().unwrap();
        cx.schedule
            .blink(cx.scheduled + cx.resources.timer.cycles())
            .unwrap();
    }

    #[task(binds = USB_HP_CAN_TX, resources = [usb_dev, serial])]
    fn usb_tx(mut cx: usb_tx::Context) {
        usb_poll(&mut cx.resources.usb_dev, &mut cx.resources.serial);
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_dev, serial])]
    fn usb_rx0(mut cx: usb_rx0::Context) {
        usb_poll(&mut cx.resources.usb_dev, &mut cx.resources.serial);
    }

    extern "C" {
        fn TAMPER();
    }
};

fn usb_poll<B: UsbBus>(usb_dev: &mut UsbDevice<'static, B>, serial: &mut SerialPort<'static, B>) {
    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 8];
        match serial.read(&mut buf) {
            Ok(size) if size > 0 => {
                // Echo back in upper case
                for c in buf[0..size].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }
                serial.write(&buf[0..size]).ok();
            }
            _ => {}
        }
    }
}
