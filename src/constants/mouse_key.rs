use serde::Serialize;

#[derive(Default)]
pub struct MouseKeyInit {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub vertical_wheel: Option<i32>,
}

impl MouseKeyInit {
    pub fn init(self) -> MouseKey {
        MouseKey {
            x: self.x,
            y: self.y,
            vertical_wheel: self.vertical_wheel,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MouseKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_wheel: Option<i32>,
}
