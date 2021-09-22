#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    EnableXPlanePhysics,
    DisableXPlanePhysics,
    Unknown,
}
