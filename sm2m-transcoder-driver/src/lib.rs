pub mod base;
pub mod devices;
pub mod driver;
pub mod error;
pub mod protocol;

#[cfg(test)]
mod tests {
    use crate::driver::UsbDriver;

    #[test]
    fn return_package_version() {
        let version = UsbDriver::version();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn return_libusb_version() {
        let version = UsbDriver::libusb_version();
        assert_eq!(version, "1.0.24.11584");
    }
}
