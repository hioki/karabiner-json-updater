use crate::virtual_key::VirtualKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SetVariable {
    pub name: VirtualKey,
    pub value: u8,
}
