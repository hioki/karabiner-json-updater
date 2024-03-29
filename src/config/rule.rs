use crate::config::manipulator::Manipulator;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}
