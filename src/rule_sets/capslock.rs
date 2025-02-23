use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("Disable CapsLock"),
        manipulators: vec![Manipulator::builder()
            .from_key_with_modifiers(K::CapsLock, FromModifier::Optional(vec![Any]))
            .to_key(K::VkNone, None)
            .build()],
    }]
}
