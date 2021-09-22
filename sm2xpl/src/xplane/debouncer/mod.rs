use std::time::Duration;

pub mod input;

mod angular;
mod boolean;
mod generic;
mod linear;
mod transparent;

const MAX_INTEGRATION_TIME: Duration = Duration::from_secs(3);
