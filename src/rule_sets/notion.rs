use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[Notion] VK2+9 -> Cmd+Shift+;"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk2(),
                ]),
                from: FromInit {
                    key_code: K::Key9,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Semicolon,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+E -> Cmd+Yen"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::E,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::International3,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+F -> Cmd+P"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::F,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::P,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+H -> Cmd+["),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::H,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::CloseBracket,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+L -> Cmd+]"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::L,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::NonUsPound,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+U -> Cmd+Shift+U"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::U,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::U,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK2+9 -> Cmd+^"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk2(),
                ]),
                from: FromInit {
                    key_code: K::Key9,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::EqualSign,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK2+0 -> Cmd+Hyphen"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Notion),
                    Condition::with_vk2(),
                ]),
                from: FromInit {
                    key_code: K::Key0,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Hyphen,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Notion] VK4+N/P -> Ctrl+Shift+J/K"),
            manipulators: vec![
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Notion),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::N,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::J,
                        modifiers: Some(vec![Ctrl, Shift]),
                    }],
                    ..Default::default()
                }
                .init(),
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Notion),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::P,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::K,
                        modifiers: Some(vec![Ctrl, Shift]),
                    }],
                    ..Default::default()
                }
                .init(),
            ],
        },
    ]
}
