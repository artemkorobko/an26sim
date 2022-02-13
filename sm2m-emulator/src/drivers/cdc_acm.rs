use core::borrow::BorrowMut;

use stm32f1xx_hal::usb;
use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

pub struct Device {
    device: UsbDevice<'static, usb::UsbBusType>,
    serial: SerialPort<'static, usb::UsbBusType>,
}

impl Device {
    pub fn new(conf: usb::Peripheral) -> Self {
        let alloc = unsafe {
            static mut USB_BUS: Option<UsbBusAllocator<usb::UsbBusType>> = None;
            *USB_BUS.borrow_mut() = Some(usb::UsbBus::new(conf));
            USB_BUS.as_ref().unwrap()
        };

        let serial = SerialPort::new(alloc);
        let device = UsbDeviceBuilder::new(alloc, UsbVidPid(0x0483, 0x5740))
            .manufacturer("FSElectronics")
            .product("An26 SM2M Emulator")
            .serial_number("SM2M-EMULATOR")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        Self { device, serial }
    }

    pub fn poll(&mut self) -> bool {
        self.device.poll(&mut [&mut self.serial])
    }

    pub fn read(&mut self, data: &mut [u8]) -> Result<usize, UsbError> {
        self.serial.read(data)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, UsbError> {
        self.serial.write(data)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<usize, UsbError> {
        let mut sent = 0;
        while sent < buf.len() {
            sent += self.write(buf)?;
        }
        Ok(sent)
    }
}
