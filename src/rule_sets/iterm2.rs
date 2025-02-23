use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("Substitute TMUX prefix with VK4 on iTerm2"),
            manipulators: vec![K::C, K::H, K::J, K::K, K::L, K::N, K::P, K::S, K::V]
                .into_iter()
                .map(|key_code| {
                    ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_vk4(),
                        ]),
                        from: FromInit {
                            key_code: key_code.clone(),
                            ..Default::default()
                        }
                        .init(),
                        to: vec![
                            To::new_tmux_prefix_key(),
                            To::Key {
                                key_code,
                                modifiers: Some(vec![Ctrl]),
                            },
                        ],
                        ..Default::default()
                    }
                    .init()
                })
                .collect_vec(),
        },
        Rule {
            description: S("Avoid Cmd+W in iTerm"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![Condition::on_app(BundleIdentifier::ITerm2)]),
                from: FromInit {
                    key_code: K::W,
                    modifiers: Some(FromModifier::Mandatory(vec![Cmd])),
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::VkNone,
                    modifiers: None,
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[iTerm2] VK1+O -> Ctrl+T Ctrl+P / VK1+P -> Ctrl+T Ctrl+N"),
            manipulators: vec![(K::O, K::P), (K::P, K::N)]
                .into_iter()
                .map(|(from, to)| {
                    ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_vk1(),
                        ]),
                        from: FromInit {
                            key_code: from,
                            ..Default::default()
                        }
                        .init(),
                        to: vec![
                            To::new_tmux_prefix_key(),
                            To::Key {
                                key_code: to,
                                modifiers: Some(vec![Ctrl]),
                            },
                        ],
                        ..Default::default()
                    }
                    .init()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK2+A -> Ctrl+T Ctrl+P / VK2+S -> Ctrl+T Ctrl+N"),
            manipulators: vec![(K::A, K::P), (K::S, K::N)]
                .into_iter()
                .map(|(from, to)| {
                    ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_vk2(),
                        ]),
                        from: FromInit {
                            key_code: from,
                            ..Default::default()
                        }
                        .init(),
                        to: vec![
                            To::new_tmux_prefix_key(),
                            To::Key {
                                key_code: to,
                                modifiers: Some(vec![Ctrl]),
                            },
                        ],
                        ..Default::default()
                    }
                    .init()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK1+W -> <ESC>:w<CR>"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::W,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::Key {
                        key_code: K::Escape,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::Quote,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::W,
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
        Rule {
            description: S("[iTerm2] VK1+Q -> <ESC>:q<CR>"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::Q,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::Key {
                        key_code: K::Escape,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::Quote,
                        modifiers: None,
                    },
                    To::Key {
                        key_code: K::Q,
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
        Rule {
            description: S("[iTerm2] VK1+Z -> Enter tmux copy-mode"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::Z,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::new_tmux_prefix_key(),
                    To::Key {
                        key_code: K::CloseBracket,
                        modifiers: Some(vec![Ctrl]),
                    },
                ],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[iTerm2] VK1+U -> Shift+0 / VK1+I -> shift+4"),
            manipulators: vec![(K::U, K::Key0), (K::I, K::Key4)]
                .into_iter()
                .map(|(from, to)| {
                    ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_vk1(),
                        ]),
                        from: FromInit {
                            key_code: from,
                            ..Default::default()
                        }
                        .init(),
                        to: vec![To::Key {
                            key_code: to,
                            modifiers: Some(vec![Shift]),
                        }],
                        ..Default::default()
                    }
                    .init()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK1+Semicolon -> Ctrl+F"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::Semicolon,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F,
                    modifiers: Some(vec![Ctrl]),
                }],
                ..Default::default()
            }
            .init()],
        },
    ]
}
