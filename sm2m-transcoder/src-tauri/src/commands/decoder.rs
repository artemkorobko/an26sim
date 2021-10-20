use crate::usb::Usb;

#[tauri::command]
pub fn decoder_find(usb: tauri::State<Usb>) -> Result<(), String> {
    let mut driver = usb.driver.lock().unwrap();
    if let Some(ref mut driver) = *driver {
        let device = driver
            .find_decoder(std::time::Duration::from_secs(1))
            .map_err(|error| error.to_string())?;

        if device.is_some() {
            *usb.decoder.lock().unwrap() = device;
            Ok(())
        } else {
            Err("Decoder not found".to_owned())
        }
    } else {
        Err("The driver has not been initialized".to_owned())
    }
}
