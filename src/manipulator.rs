use crate::condition::Condition;
use crate::from::From;
use crate::key_code::KeyCode;
use crate::set_variable::SetVariable;
use crate::to::To;
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
