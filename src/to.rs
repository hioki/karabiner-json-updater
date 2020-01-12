use crate::key_code::KeyCode;
use crate::modifier_key::ModifierKey;
use crate::mouse_key::MouseKey;
use crate::set_variable::SetVariable;
use serde::Serialize;

pub fn tmux_prefix() -> To {
    To::Key {
        key_code: KeyCode::T,
        modifiers: Some(vec![ModifierKey::Control]),
    }
}

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PointingButton {
    Button1,
    Button2,
}
