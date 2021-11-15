use std::{sync::mpsc, thread};

pub struct USBThreadHandle {
    term_tx: mpsc::Sender<()>,
    handle: Option<thread::JoinHandle<()>>,
}

impl USBThreadHandle {
    pub fn new(term_tx: mpsc::Sender<()>, handle: Option<thread::JoinHandle<()>>) -> Self {
        Self { term_tx, handle }
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
