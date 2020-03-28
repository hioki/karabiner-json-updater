use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierKey {
    Any,
    Control,
    Shift,
    #[serde(rename = "option")]
    Opt,
    Command,
}
