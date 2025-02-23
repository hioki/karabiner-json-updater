use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .from_key_with_modifiers(K::Semicolon, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(K::Semicolon, None)
            .build(),
        Manipulator::builder()
            .from_key_with_modifiers(K::Semicolon, FromModifier::Mandatory(vec![Cmd, Shift]))
            .to_key(K::KeypadPlus, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .from_key(K::Semicolon)
            .to_key(K::ReturnOrEnter, None)
            .build(),
    ]
}
