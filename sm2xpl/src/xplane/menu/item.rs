use num_derive::FromPrimitive;

#[derive(Hash, Eq, PartialEq, FromPrimitive)]
pub enum MenuItem {
    Undefined,
    Physics,
    Inspector,
    Test,
}
