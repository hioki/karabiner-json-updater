use crate::rule::Rule;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Config {
    pub title: &'static str,
    pub rules: Vec<Rule>,
}
