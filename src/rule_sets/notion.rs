use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk2_conditions = vec![
        Condition::on_app(BundleIdentifier::Notion),
        Condition::with_vk2(),
    ];
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::Notion),
        Condition::with_vk4(),
    ];
    vec![
        Manipulator::builder()
            .conditions(vk2_conditions.clone())
            .from_key(K::Key9)
            .to_key(K::Semicolon, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::E)
            .to_key(K::International3, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::F)
            .to_key(K::P, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::H)
            .to_key(K::CloseBracket, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::L)
            .to_key(K::NonUsPound, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::U)
            .to_key(K::U, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk2_conditions.clone())
            .from_key(K::Key9)
            .to_key(K::EqualSign, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk2_conditions.clone())
            .from_key(K::Key0)
            .to_key(K::Hyphen, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::N)
            .to_key(K::J, Some(vec![Ctrl, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::P)
            .to_key(K::K, Some(vec![Ctrl, Shift]))
            .build(),
    ]
}
