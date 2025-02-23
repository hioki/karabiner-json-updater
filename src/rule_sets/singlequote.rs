use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("Ctrl+Colon -> SingleQuote"),
        manipulators: vec![ManipulatorInit {
            from: From {
                key_code: K::Quote,
                modifiers: Some(FromModifier::Mandatory(vec![Ctrl])),
            },
            to: vec![To::Key {
                key_code: K::Key7,
                modifiers: Some(vec![Shift]),
            }],
            ..Default::default()
        }
        .init()],
    }]
}
