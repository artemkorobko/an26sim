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

    use rand::Rng;
    use simple_logger::SimpleLogger;

    use crate::{
        driver::{USBDevice, USBDriver},
        protocol::{Request, Response, SM2MDevice},
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
        let decoder = find_decoder_device();
        assert!(decoder.is_some(), "No decoder device found");
    }

    #[test]
    fn reset_device() {
        let mut decoder = find_decoder_device().expect("No decoder device found");
        let result = decoder.reset();
        assert!(result.is_ok(), "Error resetting device");
    }

    #[test]
    fn test_ping() {
        init_logger();
        let mut decoder = find_decoder_device().expect("No decoder device found");
        decoder.reset().expect("Error resetting device");
        for version in u16::MIN..(u8::MAX as u16 + 1) {
            let version = version as u8;
            let payload = rand::thread_rng().gen_range(5..15);
            log::info!("Send ping request {} with payload {}", version, payload);
            let size = decoder
                .write_request(Request::Ping(version, payload))
                .expect("Error sending ping request");
            assert_eq!(size, 2);
            let response = decoder
                .read_response()
                .expect("Error reading pong response");
            assert_eq!(Response::Pong(version.wrapping_add(1), payload), response);
        }
    }

    #[test]
    fn turn_led_on() {
        init_logger();
        let mut swcoder = find_decoder_device().expect("No decoder device found");
        swcoder.reset().expect("Error resetting device");
        let size = swcoder
            .write_request(Request::LedOn)
            .expect("Error sending led on request");
        assert_eq!(size, 1);
    }

    #[test]
    fn turn_led_off() {
        init_logger();
        let mut decoder = find_decoder_device().expect("No decoder device found");
        decoder.reset().expect("Error resetting device");
        let size = decoder
            .write_request(Request::LedOff)
            .expect("Error sending led off request");
        assert_eq!(size, 1);
    }

    #[test]
    fn set_get_param() {
        init_logger();
        let mut decoder = find_decoder_device().expect("No decoder device found");
        decoder.reset().expect("Unable to reset device");
        for index in 0..11 {
            let param = rand::thread_rng().gen_range(u16::MIN..u16::MAX);
            log::info!("Sending set param {} with value {}", index, param);
            let size = decoder
                .write_request(Request::SetParam(index, param))
                .expect("Error sending set param request");
            assert_eq!(size, 3);
            let size = decoder
                .write_request(Request::GetParam(index))
                .expect("Error sending get param request");
            assert_eq!(size, 1);
            let response = decoder
                .read_response()
                .expect("Error reading param response");
            assert_eq!(Response::Param(index, param), response);
        }
    }

    fn init_logger() {
        SimpleLogger::new()
            .init()
            .expect("Error initializing test logger");
    }

    fn find_decoder_device() -> Option<USBDevice> {
        let mut driver = USBDriver::new().expect("Error initializing driver");
        driver
            .find_decoder(IO_TIMEOUT)
            .expect("Error finding decoder")
    }
}
