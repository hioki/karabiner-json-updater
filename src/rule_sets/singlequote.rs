use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        Manipulator::builder()
            .from_key_with_modifiers(K::Quote, FromModifier::Mandatory(vec![Ctrl]))
            .to_key(K::Key7, Some(vec![Shift]))
            .build(),
    ]
}
