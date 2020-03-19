use crate::config::condition::Condition;
use crate::config::from::{From, FromModifier};
use crate::config::key_code::KeyCode;
use crate::config::modifier_key::ModifierKey;
use crate::config::set_variable::SetVariable;
use crate::config::to::To;
use crate::config::virtual_key::VirtualKey;
use serde::Serialize;

pub struct ManipulatorInit {
    pub conditions: Option<Vec<Condition>>,
    pub from: From,
    pub to: Vec<To>,
    pub to_delayed_action: Option<ToDelayedAction>,
    pub to_after_key_up: Option<Vec<ToAfterKeyUp>>,
    pub to_if_alone: Option<Vec<ToIfAlone>>,
}

impl Default for ManipulatorInit {
    fn default() -> Self {
        Self {
            conditions: None,
            from: From {
                key_code: KeyCode::Escape,
                modifiers: None,
            },
            to: vec![],
            to_delayed_action: None,
            to_after_key_up: None,
            to_if_alone: None,
        }
    }
}

impl ManipulatorInit {
    pub fn init(self) -> Manipulator {
        Manipulator {
            r#type: Default::default(),
            conditions: self.conditions,
            from: self.from,
            to: self.to,
            to_delayed_action: self.to_delayed_action,
            to_after_key_up: self.to_after_key_up,
            to_if_alone: self.to_if_alone,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Manipulator {
    pub r#type: ManipulatorType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,

    pub from: From,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub to: Vec<To>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_delayed_action: Option<ToDelayedAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_after_key_up: Option<Vec<ToAfterKeyUp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_if_alone: Option<Vec<ToIfAlone>>,
}

impl Manipulator {
    pub fn new_for_key_to_key_mapping(
        from: KeyCode,
        from_modifiers: Option<FromModifier>,
        to: KeyCode,
        to_modifiers: Option<Vec<ModifierKey>>,
    ) -> Manipulator {
        ManipulatorInit {
            from: From {
                key_code: from,
                modifiers: from_modifiers,
            },
            to: vec![To::Key {
                key_code: to,
                modifiers: to_modifiers,
            }],
            ..Default::default()
        }
        .init()
    }

    pub fn new_for_key_to_key_mapping_with_single_virtual_key(
        virtual_key: VirtualKey,
        from: KeyCode,
        from_modifiers: Option<FromModifier>,
        to: KeyCode,
        to_modifiers: Option<Vec<ModifierKey>>,
    ) -> Manipulator {
        ManipulatorInit {
            conditions: Some(vec![Condition::with_virtual_key(virtual_key)]),
            from: From {
                key_code: from,
                modifiers: from_modifiers,
            },
            to: vec![To::Key {
                key_code: to,
                modifiers: to_modifiers,
            }],
            ..Default::default()
        }
        .init()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManipulatorType {
    Basic,
}

#[derive(Debug, Serialize)]
pub struct ToDelayedAction {
    pub to_if_invoked: Vec<To>,
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
