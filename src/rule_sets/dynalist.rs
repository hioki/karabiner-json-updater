use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk1_conditions = vec![
        Condition::on_app(BundleIdentifier::Dynalist),
        Condition::with_vk1(),
    ];
    let vk2_conditions = vec![
        Condition::on_app(BundleIdentifier::Dynalist),
        Condition::with_vk2(),
    ];
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::Dynalist),
        Condition::with_vk4(),
    ];
    vec![
        Manipulator::builder()
            .conditions(vk1_conditions.clone())
            .from_key(K::U)
            .to_key(K::A, Some(vec![Ctrl]))
            .build(),
        Manipulator::builder()
            .conditions(vk1_conditions.clone())
            .from_key(K::I)
            .to_key(K::E, Some(vec![Ctrl]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::E)
            .to_key(K::F, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::F)
            .to_key(K::O, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk2_conditions.clone())
            .from_key(K::Key9)
            .to_key(K::Hyphen, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk2_conditions.clone())
            .from_key(K::Key0)
            .to_key(K::Hyphen, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::J)
            .to_key(K::DownArrow, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::K)
            .to_key(K::UpArrow, Some(vec![Cmd]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::H)
            .to_key(K::Tab, Some(vec![Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::L)
            .to_key(K::Tab, None)
            .build(),
    ]
}
