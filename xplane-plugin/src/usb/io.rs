use sm2m_transcoder_driver::driver::{Device, Driver};

pub struct USBIo {
    driver: Driver,
    decoder: Option<Device>,
    encoder: Option<Device>,
}

impl USBIo {
    pub fn new(driver: Driver) -> Self {
        Self {
            driver,
            decoder: None,
            encoder: None,
        }
    }

    pub fn read(&self) -> Option<Vec<u8>> {
        None
    }
}
