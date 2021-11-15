use sm2m_transcoder_driver::driver::Driver;
use std::{sync::mpsc, thread, time};

use super::{io::USBIo, thread_handle::USBThreadHandle};

pub fn start(driver: Driver) -> USBThreadHandle {
    let (term_tx, term_rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut is_running = true;
        let io = USBIo::new(driver);
        while is_running {
            match term_rx.try_recv() {
                Err(mpsc::TryRecvError::Empty) => {
                    io.read();
                    thread::sleep(time::Duration::from_secs(1));
                }
                _ => is_running = false,
            }
        }
    });

    USBThreadHandle::new(term_tx, Some(handle))
}
