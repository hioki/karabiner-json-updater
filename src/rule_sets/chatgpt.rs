use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("[ChatGPT] VK4+E -> Cmd+Ctrl+S"),
        manipulators: vec![ManipulatorInit {
            conditions: Some(vec![
                Condition::on_app(BundleIdentifier::ChatGPT),
                Condition::with_vk4(),
            ]),
            from: FromInit {
                key_code: K::E,
                ..Default::default()
            }
            .init(),
            to: vec![To::Key {
                key_code: K::S,
                modifiers: Some(vec![Cmd, Ctrl]),
            }],
            ..Default::default()
        }
        .init()],
    }]
}
