#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod usb;

fn main() {
    tauri::Builder::default()
        .manage(usb::Usb::default())
        .invoke_handler(tauri::generate_handler![
            commands::driver::driver_init,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
