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
pub struct FromModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<Vec<ModifierKey>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<Vec<ModifierKey>>,
}
