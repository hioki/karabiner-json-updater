use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MouseKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_wheel: Option<i32>,
}
