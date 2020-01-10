#[derive(Debug)]
pub enum Value {
    On,
    Off,
}

impl Value {
    pub fn value(&self) -> u8 {
        match self {
            Value::On => 1,
            Value::Off => 0,
        }
    }
}
