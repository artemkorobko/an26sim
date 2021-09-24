#[derive(thiserror::Error, Debug)]
pub enum MenuError {
    #[error("Unable to find plugins menu")]
    PluginsMenu,
    #[error("Invalid menu item text '{0}'")]
    InvalidText(String),
    #[error("Invalid parent for menu item '{0}'")]
    Append(String),
    #[error("Error creating menu '{0}'")]
    Create(String),
}
