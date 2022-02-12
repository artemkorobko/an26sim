use core::borrow::BorrowMut;

use stm32f4xx_hal::otg_fs;
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
    UsbError,
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[derive(Default)]
pub struct Descriptor {
    pub vendor_id: u16,
    pub product_id: u16,
    pub manufacturer: &'static str,
    pub product: &'static str,
    pub serial_number: &'static str,
}

pub struct Device {
    usb_dev: UsbDevice<'static, otg_fs::UsbBusType>,
    serial: SerialPort<'static, otg_fs::UsbBusType>,
}

impl Device {
    pub fn new(conf: otg_fs::USB, descriptor: Descriptor) -> Self {
        let alloc = unsafe {
            static mut EP_MEMORY: [u32; 1024] = [0; 1024];
            static mut USB_BUS: Option<UsbBusAllocator<otg_fs::UsbBusType>> = None;
            *USB_BUS.borrow_mut() = Some(otg_fs::UsbBus::new(conf, &mut EP_MEMORY));
            USB_BUS.as_ref().unwrap()
        };

        let serial = SerialPort::new(alloc);
        let usb_dev = UsbDeviceBuilder::new(alloc, UsbVidPid(0x0483, 0x5740))
            .manufacturer(descriptor.manufacturer)
            .product("An26 SM2M Decoder")
            .serial_number("SM2M-DECODER")
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64)
            .build();

        Self { usb_dev, serial }
    }

    pub fn poll(&mut self) -> bool {
        self.usb_dev.poll(&mut [&mut self.serial])
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
