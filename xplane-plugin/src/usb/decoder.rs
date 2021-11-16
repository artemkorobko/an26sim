use std::sync::mpsc;

use sm2m_transcoder_driver::{
    driver::{USBDevice, USBDriver},
    error::DriverError,
};

use super::thread::{FIND_TIMEOUT, IO_TIMEOUT};

pub enum State {
    FindDecoder,
    BeforeMarker(USBDevice),
    BeforePayload(USBDevice),
    PayloadSize(USBDevice, usize),
    AfterPayloadSize(USBDevice, usize),
    BeforeStartMarker(USBDevice, usize),
    ReadPayload(USBDevice, usize, Vec<u8>),
}

impl Default for State {
    fn default() -> Self {
        State::FindDecoder
    }
}

pub fn process_state(
    state: State,
    driver: &mut USBDriver,
    read_tx: &mpsc::Sender<Vec<u8>>,
) -> Result<State, DriverError> {
    match state {
        State::FindDecoder => find_decoder(driver),
        State::BeforeMarker(device) => before_marker(device),
        State::BeforePayload(device) => before_payload(device),
        State::PayloadSize(device, offset) => payload_size(device, offset),
        State::AfterPayloadSize(device, offset) => after_payload_size(device, offset),
        State::BeforeStartMarker(device, size) => before_start_marker(device, size),
        State::ReadPayload(device, size, payload) => read_payload(device, size, payload, read_tx),
    }
}

fn find_decoder(driver: &mut USBDriver) -> Result<State, DriverError> {
    match driver.find_decoder(FIND_TIMEOUT)? {
        Some(device) => Ok(State::BeforeMarker(device)),
        None => Ok(State::FindDecoder),
    }
}

fn before_marker(mut device: USBDevice) -> Result<State, DriverError> {
    if read_marker_byte(&mut device)? == ReadStartMarker::Found {
        Ok(State::BeforePayload(device))
    } else {
        Ok(State::BeforeMarker(device))
    }
}

fn before_payload(mut device: USBDevice) -> Result<State, DriverError> {
    match read_marker_byte(&mut device)? {
        ReadStartMarker::Found => Ok(State::PayloadSize(device, 0)),
        ReadStartMarker::NotFound => Ok(State::BeforeMarker(device)),
        ReadStartMarker::NoData => Ok(State::BeforePayload(device)),
    }
}

fn payload_size(mut device: USBDevice, offset: usize) -> Result<State, DriverError> {
    match read_marker_byte(&mut device)? {
        ReadStartMarker::Found => Ok(State::AfterPayloadSize(device, offset + 1)),
        ReadStartMarker::NotFound => Ok(State::PayloadSize(device, offset + 1)),
        ReadStartMarker::NoData => Ok(State::PayloadSize(device, offset)),
    }
}

fn after_payload_size(mut device: USBDevice, offset: usize) -> Result<State, DriverError> {
    match read_marker_byte(&mut device)? {
        ReadStartMarker::Found => Ok(State::ReadPayload(
            device,
            offset - 1,
            Vec::with_capacity(offset - 1),
        )),
        ReadStartMarker::NotFound => Ok(State::PayloadSize(device, offset + 1)),
        ReadStartMarker::NoData => Ok(State::AfterPayloadSize(device, offset)),
    }
}

fn before_start_marker(mut device: USBDevice, size: usize) -> Result<State, DriverError> {
    match read_marker(&mut device)? {
        ReadStartMarker::Found => Ok(State::ReadPayload(device, size, Vec::with_capacity(size))),
        ReadStartMarker::NotFound => Ok(State::BeforeMarker(device)),
        ReadStartMarker::NoData => Ok(State::BeforeStartMarker(device, size)),
    }
}

fn read_payload(
    mut device: USBDevice,
    size: usize,
    mut payload: Vec<u8>,
    read_tx: &mpsc::Sender<Vec<u8>>,
) -> Result<State, DriverError> {
    let mut buf = [0u8; 256];
    if device.read(&mut buf, IO_TIMEOUT)? > 0 {
        payload.extend(buf);
    }

    if payload.len() < size {
        Ok(State::ReadPayload(device, size, payload))
    } else {
        if let Err(error) = read_tx.send(payload) {
            xplm::debug!("USB decoder error: {}", error);
        }
        Ok(State::BeforeStartMarker(device, size))
    }
}

#[derive(PartialEq)]
enum ReadStartMarker {
    Found,
    NotFound,
    NoData,
}

fn read_marker_byte(device: &mut USBDevice) -> Result<ReadStartMarker, DriverError> {
    const START_MARKER: u8 = 170;
    let mut buf = [0u8; 1];
    if device.read(&mut buf, IO_TIMEOUT)? > 0 {
        if u8::from_le_bytes(buf) == START_MARKER {
            Ok(ReadStartMarker::Found)
        } else {
            Ok(ReadStartMarker::NotFound)
        }
    } else {
        Ok(ReadStartMarker::NoData)
    }
}

fn read_marker(device: &mut USBDevice) -> Result<ReadStartMarker, DriverError> {
    const START_MARKER: u16 = 43690;
    let mut buf = [0u8; 2];
    if device.read(&mut buf, IO_TIMEOUT)? > 0 {
        if u16::from_le_bytes(buf) == START_MARKER {
            Ok(ReadStartMarker::Found)
        } else {
            Ok(ReadStartMarker::NotFound)
        }
    } else {
        Ok(ReadStartMarker::NoData)
    }
}
