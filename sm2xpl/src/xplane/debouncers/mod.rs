use std::time::Duration;

pub mod angular;
pub mod boolean;
pub mod generic;
pub mod linear;

const MAX_BOUNCE_TIME: Duration = Duration::from_secs(3);