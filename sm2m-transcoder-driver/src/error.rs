use thiserror::Error;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error("can't initialize USB driver, reason: {0}")]
    Init(#[source] rusb::Error),
    #[error("can't read USB device list, reason: {0}")]
    DeviceList(#[source] rusb::Error),
    #[error("can't read device descriptor, reason: {0}")]
    ReadDeviceDescriptor(#[source] rusb::Error),
    #[error("can't open device {1}:{2}, reason: {0}")]
    OpenDevice(#[source] rusb::Error, u16, u16),
    #[error("can't read serial number for device {1}:{2}, reason: {0}")]
    SerialNumber(#[source] rusb::Error, u16, u16),
    #[error("can't read languages for device {1}:{2}, reason: {0}")]
    ReadLanguages(#[source] rusb::Error, u16, u16),
    #[error("USB device {0}:{1} does not have readable endpoint")]
    NoReadableEndpoint(u16, u16),
    #[error("USB device {0}:{1} does not have writeable endpoint")]
    NoWriteableEndpoint(u16, u16),
    #[error("can't detect kernel driver state for interface {1}, reason: {0}")]
    KernelDriverState(#[source] rusb::Error, u8),
    #[error("can't detach kernel driver for interface {1}, reason: {0}")]
    DetachKernelDriver(#[source] rusb::Error, u8),
    #[error("can't attach kernel driver for interface {1}, reason: {0}")]
    AttachKernelDriver(#[source] rusb::Error, u8),
    #[error("can't set active configuration {1}, reason: {0}")]
    ActiveConfiguration(#[source] rusb::Error, u8),
    #[error("can't claim interface {1}, reason: {0}")]
    ClaimInterface(#[source] rusb::Error, u8),
    #[error("can't set alternate setting {1} for interface {2}, reason: {0}")]
    AlternateSetting(#[source] rusb::Error, u8, u8),
    #[error("can't enable interface {1} endpoint for device {2}:{3}, reason: {0}")]
    EnableEndpoint(#[source] rusb::Error, u8, u16, u16),
    #[error("can't reset device {1}:{2}, reason: {0}")]
    Reset(#[source] rusb::Error, u16, u16),
    #[error("can't read from USB interface {1}, reason: {0}")]
    Read(#[source] rusb::Error, u8),
    #[error("can't write to USB interface {1}, reason: {0}")]
    Write(#[source] rusb::Error, u8),
    #[error("unsupported input transfer type {:?} for address {1}")]
    UnsupportedInputTransferType(rusb::TransferType, u8),
    #[error("unsupported output transfer type {:?} for address {1}")]
    UnsupportedOutputTransferType(rusb::TransferType, u8),
}
