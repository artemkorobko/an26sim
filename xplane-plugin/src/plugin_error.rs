use sm2m_transcoder_driver::error::DriverError;
use thiserror::Error;
use xplm::data::borrowed::FindError;

use crate::xplane::{inspector::error::WidgetError, menu::error::MenuError};

#[derive(Error, Debug)]
pub enum PluginError {
    #[error(transparent)]
    DataRefError(#[from] FindError),
    #[error(transparent)]
    DriverError(#[from] DriverError),
    #[error("Unable to create menu: {0}")]
    MenuError(#[from] MenuError),
    #[error("Unable to create window: {0}")]
    WidgetError(#[from] WidgetError),
}
