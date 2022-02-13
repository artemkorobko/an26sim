pub mod device;
pub mod inbound;
pub mod outbound;

pub mod prelude {
    pub use super::device::CdcDevice;
    pub use super::inbound::{Reader, UsbInbound};
    pub use super::outbound::{UsbOutbound, Writer};
}
