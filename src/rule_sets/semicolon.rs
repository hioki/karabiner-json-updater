use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("Semicolon -> Enter"),
        manipulators: vec![
            Manipulator::new_for_key_to_key_mapping(
                K::Semicolon,
                Some(FromModifier::Mandatory(vec![Ctrl])),
                K::Semicolon,
                None,
            ),
            Manipulator::new_for_key_to_key_mapping(
                K::Semicolon,
                Some(FromModifier::Mandatory(vec![Cmd, Shift])),
                K::KeypadPlus,
                Some(vec![Cmd]),
            ),
            Manipulator::new_for_key_to_key_mapping(K::Semicolon, None, K::ReturnOrEnter, None),
        ],
    }]
}
