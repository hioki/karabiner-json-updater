use crate::key_code::KeyCode;
use crate::modifier_key::ModifierKey;
use crate::mouse_key::MouseKey;
use crate::set_variable::SetVariable;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct To {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_variable: Option<SetVariable>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_code: Option<KeyCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<ModifierKey>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_key: Option<MouseKey>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointing_button: Option<PointingButton>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<&'static str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointingButton {
    Button1,
    Button2,
}
