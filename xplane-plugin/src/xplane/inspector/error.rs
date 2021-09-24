use thiserror::Error;

#[derive(Error, Debug)]
pub enum WidgetError {
    #[error("Invalid widget text '{0}'")]
    InvalidText(String),
    #[error("Error creaing window '{0}'")]
    Widget(String),
}
