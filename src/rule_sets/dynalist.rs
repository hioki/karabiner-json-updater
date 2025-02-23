use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[Dynalist] VK1+U/I -> Ctrl+A/E"),
            manipulators: vec![
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk1(),
                    ]),
                    from: FromInit {
                        key_code: K::U,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::A,
                        modifiers: Some(vec![Ctrl]),
                    }],
                    ..Default::default()
                }
                .init(),
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk1(),
                    ]),
                    from: FromInit {
                        key_code: K::I,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::E,
                        modifiers: Some(vec![Ctrl]),
                    }],
                    ..Default::default()
                }
                .init(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+E -> Toggle file pane"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Dynalist),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::E,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Dynalist] VK4+F -> Open file finder"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Dynalist),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::F,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::O,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Dynalist] VK2+9/0 -> Cmd+Shift+Hyphen/Cmd+Hyphen"),
            manipulators: vec![
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk2(),
                    ]),
                    from: FromInit {
                        key_code: K::Key9,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::Hyphen,
                        modifiers: Some(vec![Cmd, Shift]),
                    }],
                    ..Default::default()
                }
                .init(),
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
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
                .init(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+J/K -> Cmd+DownArrow/Cmd+UpArrow"),
            manipulators: vec![
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::J,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::DownArrow,
                        modifiers: Some(vec![Cmd]),
                    }],
                    ..Default::default()
                }
                .init(),
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::K,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::UpArrow,
                        modifiers: Some(vec![Cmd]),
                    }],
                    ..Default::default()
                }
                .init(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+H/L -> Shift+Tab/Tab"),
            manipulators: vec![
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::H,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::Tab,
                        modifiers: Some(vec![Shift]),
                    }],
                    ..Default::default()
                }
                .init(),
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::Dynalist),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: K::L,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code: K::Tab,
                        modifiers: None,
                    }],
                    ..Default::default()
                }
                .init(),
            ],
        },
    ]
}
