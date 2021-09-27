use core::borrow::BorrowMut;

use stm32f1xx_hal::{device, gpio, usb};
use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBusType>> = None;

pub struct CDCDevice {
    usb_dev: UsbDevice<'static, usb::UsbBusType>,
    serial: SerialPort<'static, usb::UsbBusType>,
}

impl CDCDevice {
    pub fn new(
        usb: device::USB,
        usb_dm: gpio::gpioa::PA11<gpio::Input<gpio::Floating>>,
        usb_dp: gpio::gpioa::PA12<gpio::Input<gpio::Floating>>,
    ) -> Self {
        let peripheral = usb::Peripheral {
            usb,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        };

        let (usb_dev, serial) = unsafe {
            *USB_BUS.borrow_mut() = Some(usb::UsbBus::new(peripheral));
            let serial = SerialPort::new(USB_BUS.as_ref().unwrap());
            let usb_dev = UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(1155, 22336))
                .manufacturer("STMicroelectronics")
                .product("STM32 Virtual ComPort")
                .serial_number("SM2M-DECODER")
                .device_class(USB_CLASS_CDC)
                .build();
            (usb_dev, serial)
        };

        Self { usb_dev, serial }
    }

    pub fn read(&mut self, data: &mut [u8]) -> Result<usize, UsbError> {
        if self.usb_dev.poll(&mut [&mut self.serial]) {
            self.serial.read(data)
        } else {
            Ok(0)
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, UsbError> {
        self.serial.write(data)
    }
}
