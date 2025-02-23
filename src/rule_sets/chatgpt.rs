use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::ChatGPT),
        Condition::with_vk4(),
    ];
    vec![
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(K::E)
            .to_key(K::S, Some(vec![Cmd, Ctrl]))
            .build(),
    ]
}
