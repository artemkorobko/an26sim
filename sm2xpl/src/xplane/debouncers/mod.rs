use std::time::Duration;

pub mod angular;
pub mod boolean;
pub mod generic;
pub mod linear;

const MAX_INTEGRATION_TIME: Duration = Duration::from_secs(3);
