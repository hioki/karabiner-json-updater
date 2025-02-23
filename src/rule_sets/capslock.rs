use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("Disable CapsLock"),
        manipulators: vec![ManipulatorInit {
            from: From {
                key_code: K::CapsLock,
                modifiers: Some(FromModifier::Optional(vec![Any])),
            },
            to: vec![To::Key {
                key_code: K::VkNone,
                modifiers: None,
            }],
            ..Default::default()
        }
        .init()],
    }]
}
