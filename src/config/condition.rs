use crate::config::bundle_identifier::BundleIdentifier;
use crate::config::value::Value;
use crate::config::virtual_key::VirtualKey;
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
        value: Value,
    },
}

impl Condition {
    pub fn on_app(bundle_identifier: BundleIdentifier) -> Condition {
        Condition::OnApplication {
            r#type: ConditionType::FrontmostApplicationIf,
            bundle_identifiers: vec![bundle_identifier],
        }
    }

    pub fn with_vk1() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk1)
    }

    pub fn with_vk2() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk2)
    }

    #[allow(dead_code)]
    pub fn with_vk3() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk3)
    }

    pub fn with_vk4() -> Condition {
        Self::with_virtual_key(VirtualKey::Vk4)
    }

    pub fn with_virtual_key(virtual_key: VirtualKey) -> Condition {
        Condition::WithVirtualKey {
            r#type: ConditionType::VariableIf,
            name: virtual_key,
            value: Value::On,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionType {
    FrontmostApplicationIf,
    VariableIf,
}
