pub mod cdc_acm;
pub mod cdc_acm_inbound;
pub mod cdc_acm_outbound;

pub mod prelude {
    pub use super::cdc_acm::Device;
    pub use super::cdc_acm_inbound::{Inbound, Reader};
    pub use super::cdc_acm_outbound::{Outbound, Writer};
}
