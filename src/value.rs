use serde_repr::Serialize_repr;

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum Value {
    On = 1,
    Off = 0,
}
