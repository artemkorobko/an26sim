pub mod device;
pub mod driver;
pub mod error;
pub mod protocol;

mod device_lookup;
mod device_lookup_helper;
mod endpoint_lookup;
mod endpoint_lookup_helper;

#[cfg(test)]
mod tests {
    use std::time;

    use simple_logger::SimpleLogger;

    use crate::{
        driver::{USBDevice, USBDriver},
        protocol::SM2MDevice,
    };

    const IO_TIMEOUT: time::Duration = time::Duration::from_secs(1);

    #[test]
    fn return_package_version() {
        let version = USBDriver::version();
        assert_eq!(version, "1.0.0");
    }

    #[test]
    fn return_libusb_version() {
        let version = USBDriver::libusb_version();
        assert_eq!(version, "1.0.24.11584");
    }

    #[test]
    fn find_decoder() {
        init_logger();
        if find_decoder_device().is_some() {
            log::info!("Decoder found");
        } else {
            log::error!("Decoder not found, plug-in the decoder and try test again");
        }
    }

    #[test]
    fn reset_device() {
        let device = find_decoder_device().expect("No decoder device found");
        log::info!("Resetting device...");
        device.reset().expect("Unable to reset device");
    }

    #[test]
    fn test_ping() {
        init_logger();
        let device = find_decoder_device().expect("No decoder device found");
        log::info!("Resetting device...");
        device.reset().expect("Unable to reset device");
        let mut device = find_decoder_device().expect("No decoder device found");
        log::info!("Sending ping request...");
        let payload = device.write_ping(1).expect("Error sending ping request");
        log::info!("Waiting for pong response...");
        let received = device
            .read_pong(payload)
            .expect("Error waiting for pong response");
        assert!(received);
    }

    #[test]
    fn turn_led_on() {
        init_logger();
        let device = find_decoder_device().expect("No decoder device found");
        log::info!("Resetting device...");
        device.reset().expect("Unable to reset device");
        let mut device = find_decoder_device().expect("No decoder device found");
        log::info!("Sending led on request...");
        let sent = device.led_on().expect("Error sending led on request");
        assert!(sent);
    }

    #[test]
    fn turn_led_off() {
        init_logger();
        let device = find_decoder_device().expect("No decoder device found");
        log::info!("Resetting device...");
        device.reset().expect("Unable to reset device");
        let mut device = find_decoder_device().expect("No decoder device found");
        log::info!("Sending led off request...");
        let sent = device.led_off().expect("Error sending led off request");
        assert!(sent);
    }

    fn init_logger() {
        SimpleLogger::new()
            .init()
            .expect("Error initializing test logger");
    }

    fn find_decoder_device() -> Option<USBDevice> {
        log::info!("Initializing driver...");
        let mut driver = USBDriver::new().expect("Error initializing USB driver");
        log::info!("Searching for decoder...");
        driver
            .find_decoder(IO_TIMEOUT)
            .expect("Error searching for decoder device")
    }
}
