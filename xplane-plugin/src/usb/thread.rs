use sm2m_transcoder_driver::{driver::USBDriver, error::DriverError};
use std::{sync::mpsc, thread, time};

use super::{decoder, thread_handle::USBThreadHandle};

pub const FIND_TIMEOUT: time::Duration = time::Duration::from_millis(50);
pub const IO_TIMEOUT: time::Duration = time::Duration::from_millis(10);

pub fn start(driver: USBDriver) -> USBThreadHandle {
    let (term_tx, term_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();
    let (read_tx, read_rx) = mpsc::channel();
    let handle = thread::spawn(move || thread_loop(driver, term_rx, write_rx, read_tx));
    USBThreadHandle::new(term_tx, write_tx, read_rx, Some(handle))
}

fn thread_loop(
    mut driver: USBDriver,
    term_rx: mpsc::Receiver<()>,
    write_rx: mpsc::Receiver<Vec<u8>>,
    read_tx: mpsc::Sender<Vec<u8>>,
) {
    let mut decoder_state = decoder::State::default();

    loop {
        match term_rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {
                match decoder::process_state(decoder_state, &mut driver, &read_tx) {
                    Ok(new_state) => decoder_state = new_state,
                    Err(error) => {
                        xplm::debugln!("USB thread error: {:?}", error);
                        decoder_state = decoder::State::default();
                    }
                }
            }
            _ => return,
        }
    }
}
