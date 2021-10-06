pub mod device;
pub mod driver;
pub mod error;

mod device_lookup;
mod device_lookup_helper;
mod endpoint_lookup;
mod endpoint_lookup_helper;

#[cfg(test)]
mod tests {
    use std::time;

    use simple_logger::SimpleLogger;

    use crate::driver::Driver;

    #[test]
    fn return_package_version() {
        let version = Driver::version();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn return_libusb_version() {
        let version = Driver::libusb_version();
        assert_eq!(version, "1.0.24.11584");
    }

    #[test]
    fn find_decoder() {
        SimpleLogger::new().init().unwrap();

        let mut driver = Driver::new().unwrap();
        let device = driver.find_decoder(time::Duration::from_secs(1)).unwrap();

        match device {
            Some(mut device) => {
                log::info!("Checking device");
                assert!(device.check(time::Duration::from_secs(1)).unwrap());
                log::info!("Device is ready");
            }
            None => log::error!("Decoder not found, plug-in the decoder and try test again"),
        }
    }
}
