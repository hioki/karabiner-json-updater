use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::F)
                .to_key(K::Tab, Some(vec![Cmd]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::D)
                .to_key(K::Tab, Some(vec![Cmd, Shift]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::S)
                .to_key(K::Tab, Some(vec![Ctrl]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::A)
                .to_key(K::Tab, Some(vec![Ctrl, Shift]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key9)
                .to_key(K::KeypadPlus, Some(vec![Cmd]))
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key0)
                .to_key(K::Hyphen, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key1)
                .to_key(K::VolumeDecrement, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key2)
                .to_key(K::VolumeIncrement, None)
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key3)
                .to_key(K::DisplayBrightnessDecrement, None)
                .build(),
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(K::Key4)
                .to_key(K::DisplayBrightnessIncrement, None)
                .build(),
        ],
        vec![
            (K::H, K::LeftArrow),
            (K::O, K::RightArrow),
            (K::N, K::DownArrow),
            (K::P, K::UpArrow),
            (K::U, K::Key1),
            (K::I, K::Key2),
            (K::M, K::Key3),
            (K::Comma, K::Key4),
            (K::J, K::P),
            (K::K, K::N),
        ]
        .into_iter()
        .map(|(from, to)| {
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key_with_modifiers(from, FromModifier::Mandatory(vec![Ctrl]))
                .to_key(to, Some(vec![Cmd, Ctrl, Opt, Shift]))
                .build()
        })
        .collect(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
