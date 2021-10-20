use crate::usb::Usb;
use sm2m_transcoder_driver::driver::Driver;

#[derive(serde::Serialize)]
pub struct InitResult {
    driver: String,
    libusb: String,
}

#[tauri::command]
pub fn driver_init(usb: tauri::State<Usb>) -> Result<InitResult, String> {
    let driver = Driver::new().map_err(|error| error.to_string())?;
    *usb.driver.lock().unwrap() = Some(driver);
    Ok(InitResult {
        driver: Driver::version(),
        libusb: Driver::libusb_version(),
    })
}
