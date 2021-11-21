use std::{sync::mpsc, thread};

pub struct USBThreadHandle {
    term_tx: mpsc::Sender<()>,
    write_tx: mpsc::Sender<Vec<u8>>,
    read_rx: mpsc::Receiver<Vec<u8>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl USBThreadHandle {
    pub fn new(
        term_tx: mpsc::Sender<()>,
        write_tx: mpsc::Sender<Vec<u8>>,
        read_rx: mpsc::Receiver<Vec<u8>>,
        handle: Option<thread::JoinHandle<()>>,
    ) -> Self {
        Self {
            term_tx,
            write_tx,
            read_rx,
            handle,
        }
    }

    pub fn read(&self) -> Option<Vec<u8>> {
        self.read_rx.try_recv().ok()
    }

    pub fn write(&self, payload: Vec<u8>) -> bool {
        self.write_tx.send(payload).is_ok()
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            self.term_tx
                .send(())
                .expect("Error sending USB thread termination signal");
            handle
                .join()
                .expect("Error waiting for USB thread completion");
        }
    }
}

impl Drop for USBThreadHandle {
    fn drop(&mut self) {
        self.stop();
    }
}
