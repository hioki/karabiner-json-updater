use crate::bundle_identifier::BundleIdentifier;
use crate::value::Value;
use crate::virtual_key::VirtualKey;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Condition {
    OnApplication {
        r#type: ConditionType,
        bundle_identifiers: Vec<BundleIdentifier>,
    },
    WithVirtualKey {
        r#type: ConditionType,
        name: VirtualKey,
        value: u8,
    },
}

impl Condition {
    pub fn on_app(bundle_identifier: BundleIdentifier) -> Condition {
        Condition::OnApplication {
            r#type: ConditionType::FrontmostApplicationIf,
            bundle_identifiers: vec![bundle_identifier],
        }
    }

    pub fn with_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition::WithVirtualKey {
            r#type: ConditionType::VariableIf,
            name: virtual_key,
            value: Value::On.value(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    FrontmostApplicationIf,
    VariableIf,
}
