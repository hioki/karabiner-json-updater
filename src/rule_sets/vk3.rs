use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        (K::A, K::Key1),
        (K::S, K::Key2),
        (K::D, K::Key3),
        (K::F, K::Key4),
        (K::G, K::Key5),
        (K::H, K::Key6),
        (K::J, K::Key7),
        (K::K, K::Key8),
        (K::L, K::Key9),
        (K::Semicolon, K::Key0),
        (K::Quote, K::Hyphen),
    ]
    .into_iter()
    .map(|(from, to)| {
        Manipulator::builder()
            .condition(Condition::with_vk3())
            .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
            .to_key(to, None)
            .build()
    })
    .collect()
}
