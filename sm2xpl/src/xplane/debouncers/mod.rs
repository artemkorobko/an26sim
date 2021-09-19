use std::time::Duration;

pub mod input;

mod angular;
mod boolean;
mod generic;
mod linear;

const MAX_BOUNCE_TIME: Duration = Duration::from_secs(3);
