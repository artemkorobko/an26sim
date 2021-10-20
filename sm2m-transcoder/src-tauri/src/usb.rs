use sm2m_transcoder_driver::driver::{DefaultDevice, Driver};
use std::sync;

#[derive(Default)]
pub struct Usb {
    pub driver: sync::Mutex<Option<Driver>>,
    pub decoder: sync::Mutex<Option<DefaultDevice>>,
    pub encoder: sync::Mutex<Option<DefaultDevice>>,
}
