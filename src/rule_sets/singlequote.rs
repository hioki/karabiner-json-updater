use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("Ctrl+Colon -> SingleQuote"),
        manipulators: vec![Manipulator::builder()
            .from_key_with_modifiers(K::Quote, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(K::Key7, Some(vec![Shift]))
            .build()],
    }]
}
