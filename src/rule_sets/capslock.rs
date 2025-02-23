use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .from_key_with_modifiers(K::CapsLock, FromModifier::Optional(vec![Any]))
            .to_key(K::VkNone, None)
            .build(),
    ]
}
