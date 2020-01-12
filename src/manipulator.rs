use crate::condition::Condition;
use crate::from::{From, FromModifier};
use crate::key_code::KeyCode;
use crate::modifier_key::ModifierKey;
use crate::set_variable::SetVariable;
use crate::to::To;
use crate::virtual_key::VirtualKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Manipulator {
    pub r#type: ManipulatorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    pub from: From,
    pub to: Vec<To>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToAfterKeyUp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<ToIfAlone>>,
}

impl Manipulator {
    pub fn new_for_key_to_key_mapping_with_single_virtual_key(
        virtual_key: VirtualKey,
        from: KeyCode,
        from_modifiers: Option<FromModifier>,
        to: KeyCode,
        to_modifiers: Option<Vec<ModifierKey>>,
    ) -> Manipulator {
        Manipulator {
            conditions: Some(vec![Condition::with_virtual_key(virtual_key)]),
            from: From {
                key_code: from,
                modifiers: from_modifiers,
            },
            to: vec![To::Key {
                key_code: to,
                modifiers: to_modifiers,
            }],
            r#type: ManipulatorType::default(),
            to_after_key_up: None,
            to_if_alone: None,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManipulatorType {
    Basic,
}

#[derive(Debug, Serialize)]
pub struct ToIfAlone {
    pub key_code: KeyCode,
}

#[derive(Debug, Serialize)]
pub struct ToAfterKeyUp {
    pub set_variable: SetVariable,
}

impl Default for ManipulatorType {
    fn default() -> Self {
        ManipulatorType::Basic
    }
}
