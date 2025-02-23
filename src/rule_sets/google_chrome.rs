use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::GoogleChrome),
        Condition::with_vk4(),
    ];
    vec![
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::M)
            .to_key(K::M, Some(vec![Cmd, Shift]))
            .build(),
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::N)
            .to_key(K::M, Some(vec![Cmd, Shift]))
            .to_key(K::DownArrow, None)
            .to_key(K::DownArrow, None)
            .to_key(K::DownArrow, None)
            .to_key(K::ReturnOrEnter, None)
            .build(),
    ]
}
