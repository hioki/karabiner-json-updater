use crate::bundle_identifier::BundleIdentifier;
use crate::value::Value;
use crate::virtual_key::VirtualKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Condition {
    pub r#type: ConditionType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_identifiers: Option<Vec<BundleIdentifier>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<VirtualKey>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<u8>,
}

impl Condition {
    pub fn on_app(bundle_identifier: BundleIdentifier) -> Condition {
        Condition {
            r#type: ConditionType::FrontmostApplicationIf,
            bundle_identifiers: Some(vec![bundle_identifier]),
            name: None,
            value: None,
        }
    }

    pub fn with_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition {
            r#type: ConditionType::VariableIf,
            bundle_identifiers: None,
            name: Some(virtual_key),
            value: Some(Value::On.value()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    FrontmostApplicationIf,
    VariableIf,
}
