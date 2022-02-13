pub mod cdc_acm;
pub mod cdc_acm_inbound;
pub mod cdc_acm_outbound;

pub mod prelude {
    pub use super::cdc_acm::CdcDevice;
    pub use super::cdc_acm_inbound::{Reader, UsbInbound};
    pub use super::cdc_acm_outbound::{UsbOutbound, Writer};
}
