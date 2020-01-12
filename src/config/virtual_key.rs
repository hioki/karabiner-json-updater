use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VirtualKey {
    Vk1,
    Vk2,
    Vk3,
    Vk4,
}
