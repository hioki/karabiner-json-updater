use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierKey {
    Any,
    #[serde(rename = "control")]
    Ctrl,
    Shift,
    #[serde(rename = "option")]
    Opt,
    Command,
}
