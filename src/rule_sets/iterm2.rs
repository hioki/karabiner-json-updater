use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk1_conditions = vec![
        Condition::on_app(BundleIdentifier::ITerm2),
        Condition::with_vk1(),
    ];
    let vk2_conditions = vec![
        Condition::on_app(BundleIdentifier::ITerm2),
        Condition::with_vk2(),
    ];
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::ITerm2),
        Condition::with_vk4(),
    ];
    vec![
        vec![K::C, K::H, K::J, K::K, K::L, K::N, K::P, K::S, K::V]
            .into_iter()
            .map(|key_code| {
                Manipulator::builder()
                    .conditions(vk4_conditions.clone())
                    .from_key(key_code.clone())
                    .to_key(KeyCode::T, Some(vec![ModifierKey::Ctrl]))
                    .to_key(key_code, Some(vec![Ctrl]))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .from_key_with_modifiers(K::W, FromModifier::Mandatory(vec![Cmd]))
                .to_key(K::VkNone, None)
                .build(),
        ],
        vec![(K::O, K::P), (K::P, K::N)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vk1_conditions.clone())
                    .from_key(from)
                    .to_key(KeyCode::T, Some(vec![Ctrl]))
                    .to_key(to, Some(vec![Ctrl]))
                    .build()
            })
            .collect(),
        vec![(K::A, K::P), (K::S, K::N)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vk2_conditions.clone())
                    .from_key(from)
                    .to_key(KeyCode::T, Some(vec![Ctrl]))
                    .to_key(to, Some(vec![Ctrl]))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .conditions(vk1_conditions.clone())
                .from_key(K::W)
                .to_key(K::Escape, None)
                .to_key(K::Quote, None)
                .to_key(K::W, None)
                .to_key(K::ReturnOrEnter, None)
                .build(),
            Manipulator::builder()
                .conditions(vk1_conditions.clone())
                .from_key(K::Q)
                .to_key(K::Escape, None)
                .to_key(K::Quote, None)
                .to_key(K::Q, None)
                .to_key(K::ReturnOrEnter, None)
                .build(),
        ],
        vec![(K::U, K::Key0), (K::I, K::Key4)]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::builder()
                    .conditions(vk1_conditions.clone())
                    .from_key(from)
                    .to_key(to, Some(vec![Shift]))
                    .build()
            })
            .collect(),
        vec![
            Manipulator::builder()
                .conditions(vk1_conditions.clone())
                .from_key(K::Semicolon)
                .to_key(K::F, Some(vec![Ctrl]))
                .build(),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}
