use crate::config::key_code::KeyCode;
use crate::config::modifier_key::ModifierKey;
use crate::config::mouse_key::MouseKey;
use crate::config::set_variable::SetVariable;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum To {
    Variable {
        set_variable: SetVariable,
    },
    Key {
        key_code: KeyCode,
        #[serde(skip_serializing_if = "Option::is_none")]
        modifiers: Option<Vec<ModifierKey>>,
    },
    Mouse {
        mouse_key: MouseKey,
    },
    Click {
        pointing_button: PointingButton,
    },
    Command {
        shell_command: &'static str,
    },
}

impl To {
    pub fn new_tmux_prefix_key() -> To {
        To::Key {
            key_code: KeyCode::T,
            modifiers: Some(vec![ModifierKey::Control]),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointingButton {
    Button1,
    Button2,
}