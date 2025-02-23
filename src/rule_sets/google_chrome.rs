use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[GoogleChrome] VK4+M -> Cmd+Shift+M (Switch profile)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::GoogleChrome),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::M,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::M,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S(
                "[GoogleChrome] VK4+N -> Cmd+Shift+M,Down,Down,Down,Enter (Toggle profile)",
            ),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::GoogleChrome),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::N,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::Key {
                        key_code: K::M,
                        modifiers: Some(vec![Cmd, Shift]),
                    },
                    To::Key {
                        key_code: K::DownArrow,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::DownArrow,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::DownArrow,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::ReturnOrEnter,
                        modifiers: None,
                    },
                ],
                ..Default::default()
            }
            .init()],
        },
    ]
}
