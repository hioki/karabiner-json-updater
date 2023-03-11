use crate::config::{key_code::KeyCode, modifier_key::ModifierKey};
use serde::Serialize;

pub struct FromInit {
    pub key_code: KeyCode,
    pub modifiers: Option<FromModifier>,
}

impl Default for FromInit {
    fn default() -> Self {
        Self {
            key_code: KeyCode::Escape,
            modifiers: None,
        }
    }
}

impl FromInit {
    pub fn init(self) -> From {
        From {
            key_code: self.key_code,
            modifiers: self.modifiers,
        }
    }
}

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
