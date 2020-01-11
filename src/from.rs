use crate::key_code::KeyCode;
use crate::modifier_key::ModifierKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct From {
    pub key_code: KeyCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<FromModifier>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FromModifier {
    Optional(Vec<ModifierKey>),
    Mandatory(Vec<ModifierKey>),
}
