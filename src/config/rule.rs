use crate::config::manipulator::Manipulator;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Rule {
    pub description: &'static str,
    pub manipulators: Vec<Manipulator>,
}
