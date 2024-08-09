use crate::config::{
    bundle_identifier::BundleIdentifier,
    condition::Condition,
    from::{From, FromInit, FromModifier},
    key_code::KeyCode as K,
    manipulator::{Manipulator, ManipulatorInit, ToAfterKeyUp, ToIfAlone},
    modifier_key::ModifierKey::*,
    mouse_key::MouseKeyInit,
    rule::Rule,
    set_variable::SetVariable,
    to::{PointingButton, To},
    value::Value,
    virtual_key::VirtualKey as VK,
};
use big_s::S;
use itertools::Itertools;

pub struct CustomRulesBuilder {}

impl CustomRulesBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Vec<Rule> {
        vec![
            rules_virtual_keys(),
            rules_iterm2(),
            rules_vscode(),
            rules_clion(),
            rules_idea_eap(),
            rules_dynalist(),
            rules_atom(),
            rules_slack(),
            rules_google_chrome(),
            rules_notion(),
            rules_clickup(),
            rules_vk1(),
            rules_vk2(),
            rules_open_apps(),
            rules_vk3(),
            rules_semicolon(),
            rules_singlequote(),
            rules_disable_capslock(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn rules_virtual_keys() -> Vec<Rule> {
    vec![Rule {
        description: S("Virtual Keys"),
        manipulators: vec![
            (K::Lang1, VK::Vk1, Some(K::JapaneseKana)),
            (K::International4, VK::Vk1, Some(K::JapaneseKana)),
            (K::Lang2, VK::Vk2, Some(K::JapaneseEisuu)),
            (K::International5, VK::Vk2, Some(K::JapaneseEisuu)),
            (K::RightGui, VK::Vk3, None),
            (K::International2, VK::Vk3, None),
            (K::Tab, VK::Vk4, Some(K::Tab)),
        ]
        .into_iter()
        .map(|(key_code, virtual_key, to_if_alone)| {
            ManipulatorInit {
                from: From {
                    key_code,
                    modifiers: Some(FromModifier::Optional(vec![Any])),
                },
                to: vec![To::Variable {
                    set_variable: SetVariable {
                        name: virtual_key.clone(),
                        value: Value::On,
                    },
                }],
                to_after_key_up: Some(vec![ToAfterKeyUp {
                    set_variable: SetVariable {
                        name: virtual_key,
                        value: Value::Off,
                    },
                }]),
                to_if_alone: to_if_alone.map(|key_code| vec![ToIfAlone { key_code }]),
                ..Default::default()
            }
            .init()
        })
        .collect_vec(),
    }]
}

fn rules_iterm2() -> Vec<Rule> {
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

fn rules_vscode() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK4 on VSCode"),
            manipulators: vec![
                K::A,    // execute command
                K::B,    // show bookmarks
                K::F,    // search file
                K::G,    // GitLens: Open File on Remote
                K::H,    // Go Back
                K::E,    // switch focus between editor and explorer
                K::L,    // Go Forward
                K::Key0, // toggle panel
                K::O,    // open recent
                K::K,    // find in path
                K::R,    // reload window
                K::S,    // go to symbol
                K::I,    // 実装へ移動
                K::Y,    // Toggle File Blame
                K::Key9, // 表示の拡大
                K::Key0, // 表示の縮小
            ]
            .into_iter()
            .map(|key_code| {
                ManipulatorInit {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::VSCode),
                        Condition::with_vk4(),
                    ]),
                    from: FromInit {
                        key_code: key_code.clone(),
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Key {
                        key_code,
                        modifiers: Some(vec![Ctrl, Shift, Opt, Cmd]),
                    }],
                    ..Default::default()
                }
                .init()
            })
            .collect_vec(),
        },
        Rule {
            description: S("[VSCode] VK4+J -> Cmd+S (Save All)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::J,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK1+W -> Cmd+S (Save All)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::W,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK4+M -> Opt+Cmd+K (Bookmarks: Toggle)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::M,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::K,
                    modifiers: Some(vec![Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK4+U -> Shift+F12 (Go To Reference)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::U,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F12,
                    modifiers: Some(vec![Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK4+N -> Opt+F8 (次の問題へ移動)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::N,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F8,
                    modifiers: Some(vec![Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK4+. -> Cmd+. (クイック修正)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Period,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Period,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[VSCode] VK4+T -> Cmd+T (全体シンボル検索)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::T,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::T,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
    ]
}

fn rules_intellij_idea(label: &str, bundle_identifier: BundleIdentifier) -> Vec<Rule> {
    vec![
        Rule {
            description: format!("[{}] ¥ -> \\", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![Condition::on_app(bundle_identifier.clone())]),
                from: FromInit {
                    key_code: K::International3,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::International3,
                    modifiers: Some(vec![Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+E -> Cmd+1 (Project)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::E,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Key1,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+F -> Cmd+Shift+O (Files)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::F,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::O,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+K -> Cmd+Shift+F (Find in Path)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::K,
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
            description: format!("[{}] VK4+N -> F2 (Next Problems)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::N,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F2,
                    modifiers: None,
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+P -> Shift+F2 (Previous Problems)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::P,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F2,
                    modifiers: Some(vec![Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+; -> Opt+Enter (More Actions)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Semicolon,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::ReturnOrEnter,
                    modifiers: Some(vec![Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+R -> Ctrl+Opt+R (Run)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::R,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::R,
                    modifiers: Some(vec![Ctrl, Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+T -> Opt+Cmd+B (Type)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::T,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::B,
                    modifiers: Some(vec![Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+S -> Opt+Cmd+O (Symbols)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::S,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::O,
                    modifiers: Some(vec![Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+C -> Cmd+O (Classes)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::C,
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
            description: format!(
                "[{}] VK4+D -> Cmd+Opt+Shift+Ctrl+D (Split and Move Down)",
                label
            ),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::D,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::D,
                    modifiers: Some(vec![Cmd, Opt, Shift, Ctrl]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!(
                "[{}] VK4+U -> Cmd+Opt+Shift+Ctrl+U (Move To Opposite Group)",
                label
            ),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::U,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::U,
                    modifiers: Some(vec![Cmd, Opt, Shift, Ctrl]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+G -> Opt+Tab (Goto Next Splitter)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::G,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Tab,
                    modifiers: Some(vec![Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+A -> Cmd+Shift+A (Actions)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::A,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::A,
                    modifiers: Some(vec![Shift, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+H -> Cmd+[", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
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
            description: format!("[{}] VK4+L -> Cmd+]", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::L,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Backslash,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+[ -> Opt+Cmd+[", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::CloseBracket,
                    modifiers: Some(FromModifier::Optional(vec![Shift])),
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::CloseBracket,
                    modifiers: Some(vec![Cmd, Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+] -> Opt+Cmd+]", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Backslash,
                    modifiers: Some(FromModifier::Optional(vec![Shift])),
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Backslash,
                    modifiers: Some(vec![Cmd, Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+M -> F3 (Bookmark)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::M,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F3,
                    modifiers: None,
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+, -> Ctrl+Shift+Opt+, (Previous Bookmark)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Comma,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Comma,
                    modifiers: Some(vec![Ctrl, Shift, Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+. -> Ctrl+Shift+Opt+. (Next Bookmark)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Period,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Period,
                    modifiers: Some(vec![Ctrl, Shift, Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+B -> Cmd+F3 (Show Bookmarks)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::B,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F3,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+O -> Ctrl+Shift+Opt+Cmd+O (Open Recent)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::O,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::O,
                    modifiers: Some(vec![Ctrl, Shift, Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+J -> Cmd+S (Save All)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::J,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK1+W -> Cmd+S (Save All)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::W,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::Key {
                        key_code: K::S,
                        modifiers: Some(vec![Cmd]),
                    },
                    To::Key {
                        key_code: K::Escape,
                        modifiers: None,
                    },
                ],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+@ -> F1 (Quick Documentation)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::OpenBracket,
                    modifiers: Some(FromModifier::Optional(vec![Shift])),
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::F1,
                    modifiers: None,
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!(
                "[{}] VK4+/ -> Ctrl+Shift+Opt+Cmd+S (Search Everywhere)",
                label
            ),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Slash,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Ctrl, Shift, Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+: -> Cmd+Shift+E (Recent Locations)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Quote,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::E,
                    modifiers: Some(vec![Shift, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+X -> Ctrl+Shift+Cmd+X (Close Project)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::X,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::X,
                    modifiers: Some(vec![Ctrl, Shift, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+I -> Opt+Cmd+B (Go to implementation)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::I,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::B,
                    modifiers: Some(vec![Opt, Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+Space -> Opt+Space (Quick Definition)", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::Spacebar,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::Spacebar,
                    modifiers: Some(vec![Opt]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+1/2 -> Opt+Cmd DownArrow/UpArrow", label),
            manipulators: vec![(K::Key1, K::UpArrow), (K::Key2, K::DownArrow)]
                .into_iter()
                .map(|(from, to)| {
                    ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(bundle_identifier.clone()),
                            Condition::with_vk4(),
                        ]),
                        from: FromInit {
                            key_code: from,
                            ..Default::default()
                        }
                        .init(),
                        to: vec![To::Key {
                            key_code: to,
                            modifiers: Some(vec![Opt, Cmd]),
                        }],
                        ..Default::default()
                    }
                    .init()
                })
                .collect_vec(),
        },
        Rule {
            description: format!("[{}] VK4+_ -> Run File Watchers", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier.clone()),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::International1,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::International1,
                    modifiers: Some(vec![Opt, Cmd, Shift, Ctrl]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: format!("[{}] VK4+Enter -> Run", label),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(bundle_identifier),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::ReturnOrEnter,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::ReturnOrEnter,
                    modifiers: Some(vec![Opt, Cmd, Shift, Ctrl]),
                }],
                ..Default::default()
            }
            .init()],
        },
    ]
}

fn rules_clion() -> Vec<Rule> {
    rules_intellij_idea("CLion", BundleIdentifier::CLion)
}

fn rules_idea_eap() -> Vec<Rule> {
    rules_intellij_idea("IdeaEAP", BundleIdentifier::JetBrainsClient)
}

fn rules_dynalist() -> Vec<Rule> {
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

fn rules_atom() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[Atom] VK4+F -> Cmd+T"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::F,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::T,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Atom] VK4+E -> Cmd+K Cmd+B"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::E,
                    ..Default::default()
                }
                .init(),
                to: vec![
                    To::Key {
                        key_code: K::K,
                        modifiers: Some(vec![Cmd]),
                    },
                    To::Key {
                        key_code: K::B,
                        modifiers: Some(vec![Cmd]),
                    },
                ],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Atom] VK4+K -> Cmd+Shift+F"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::K,
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
            description: S("[Atom] VK4+A -> Cmd+Shift+P"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::A,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::P,
                    modifiers: Some(vec![Cmd, Shift]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Atom] VK4+J -> Cmd+S"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::J,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("[Atom] VK1+W -> Cmd+S"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Atom),
                    Condition::with_vk1(),
                ]),
                from: FromInit {
                    key_code: K::W,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: K::S,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
    ]
}

fn rules_slack() -> Vec<Rule> {
    vec![Rule {
        description: S("Slack"),
        manipulators: vec![
            (K::T, K::T, vec![Cmd, Shift]),           // Threads
            (K::U, K::A, vec![Cmd, Shift]),           // All Unreads
            (K::E, K::D, vec![Cmd, Shift]),           // Toggle Sidebar
            (K::K, K::G, vec![Cmd]),                  // Search
            (K::F, K::K, vec![Cmd]),                  // Jump
            (K::B, K::S, vec![Cmd, Shift]),           // Bookmarks
            (K::D, K::X, vec![Cmd, Shift]),           // Strike through
            (K::OpenBracket, K::C, vec![Cmd, Shift]), // Code
            (K::C, K::C, vec![Cmd, Opt, Shift]),      // Code Block
            (K::Q, K::Key9, vec![Cmd, Shift]),        // Quote
        ]
        .into_iter()
        .map(|(from, to, modifiers)| {
            ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::Slack),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: from,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Key {
                    key_code: to,
                    modifiers: Some(modifiers),
                }],
                ..Default::default()
            }
            .init()
        })
        .collect_vec(),
    }]
}

fn rules_google_chrome() -> Vec<Rule> {
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

fn rules_notion() -> Vec<Rule> {
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

fn rules_clickup() -> Vec<Rule> {
    vec![Rule {
        description: S("[ClickUp] VK4+H -> Cmd+["),
        manipulators: vec![ManipulatorInit {
            conditions: Some(vec![
                Condition::on_app(BundleIdentifier::ClickUp),
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
    }]
}

fn rules_vk1() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK1+{H/J/K/L} -> {Left/Down/Up/Right}Arrow"),
            manipulators: vec![
                (K::H, K::LeftArrow),
                (K::J, K::DownArrow),
                (K::K, K::UpArrow),
                (K::L, K::RightArrow),
            ]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    from,
                    Some(FromModifier::Optional(vec![Any])),
                    to,
                    None,
                )
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+F -> Escape"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::F,
                    Some(FromModifier::Optional(vec![Any])),
                    K::Escape,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK1+S -> JapaneseKana / VK1+D -> JapaneseEisuu"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::S,
                    None,
                    K::JapaneseKana,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::D,
                    None,
                    K::JapaneseEisuu,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK1+A -> F10 / VK1+Z -> F7"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::A,
                    None,
                    K::F10,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::Z,
                    None,
                    K::F7,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK1+U -> Cmd+LeftArrow / VK1+I -> Cmd+RightArrow"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::U,
                    Some(FromModifier::Optional(vec![Any])),
                    K::LeftArrow,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::I,
                    Some(FromModifier::Optional(vec![Any])),
                    K::RightArrow,
                    Some(vec![Cmd]),
                ),
            ],
        },
        Rule {
            description: S("VK1+G -> Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::G,
                    Some(FromModifier::Optional(vec![Any])),
                    K::Tab,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK1+O -> Ctrl+Shift+Tab / VK1+P -> Ctrl+Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::O,
                    None,
                    K::Tab,
                    Some(vec![Ctrl, Shift]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::P,
                    None,
                    K::Tab,
                    Some(vec![Ctrl]),
                ),
            ],
        },
        Rule {
            description: S("VK1+Y -> Cmd+C / VK1+T -> Cmd+X / VK1+X -> Cmd+Shift+V"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::Y,
                    None,
                    K::C,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::T,
                    None,
                    K::X,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::X,
                    None,
                    K::V,
                    Some(vec![Cmd, Shift, Opt]),
                ),
            ],
        },
        Rule {
            description: S("VK1+C -> Backspace / VK1+E -> Delete"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::C,
                    None,
                    K::DeleteOrBackspace,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::E,
                    None,
                    K::DeleteForward,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK1+[ -> Cmd+Z"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::CloseBracket,
                    Some(FromModifier::Optional(vec![Any])),
                    K::Z,
                    Some(vec![Cmd]),
                ),
            ],
        },
        Rule {
            description: S("VK1+Colon -> Cmd+H"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::Quote,
                    None,
                    K::H,
                    Some(vec![Cmd]),
                ),
            ],
        },
        Rule {
            description: S("VK1+{N,M,Comma,Period} -> Mouse{Left,Down,Up,Right}"),
            manipulators: vec![
                (
                    K::N,
                    Some(FromModifier::Mandatory(vec![Shift])),
                    Some(-1536),
                    None,
                ),
                (
                    K::M,
                    Some(FromModifier::Mandatory(vec![Shift])),
                    None,
                    Some(1536),
                ),
                (
                    K::Comma,
                    Some(FromModifier::Mandatory(vec![Shift])),
                    None,
                    Some(-1536),
                ),
                (
                    K::Period,
                    Some(FromModifier::Mandatory(vec![Shift])),
                    Some(1536),
                    None,
                ),
                (K::N, None, Some(-3072), None),
                (K::M, None, None, Some(3072)),
                (K::Comma, None, None, Some(-3072)),
                (K::Period, None, Some(3072), None),
            ]
            .into_iter()
            .map(|(key_code, modifiers, x, y)| {
                ManipulatorInit {
                    conditions: Some(vec![Condition::with_vk1()]),
                    from: From {
                        key_code,
                        modifiers,
                    },
                    to: vec![To::Mouse {
                        mouse_key: MouseKeyInit {
                            x,
                            y,
                            ..Default::default()
                        }
                        .init(),
                    }],
                    ..Default::default()
                }
                .init()
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+Slash -> LeftClick / VK1+Underscore -> RightClick"),
            manipulators: vec![
                (K::Slash, PointingButton::Button1),
                (K::International1, PointingButton::Button2),
            ]
            .into_iter()
            .map(|(from, to)| {
                ManipulatorInit {
                    conditions: Some(vec![Condition::with_vk1()]),
                    from: From {
                        key_code: from,
                        modifiers: Some(FromModifier::Optional(vec![Any])),
                    },
                    to: vec![To::Click {
                        pointing_button: to,
                    }],
                    ..Default::default()
                }
                .init()
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+@ -> ScrollUp / VK1+] -> ScrollDown"),
            manipulators: vec![
                (K::OpenBracket, -64),
                (K::NonUsPound, 64),
                (K::Backslash, 64),
            ]
            .into_iter()
            .map(|(key_code, vertical_wheel)| {
                ManipulatorInit {
                    conditions: Some(vec![Condition::with_vk1()]),
                    from: FromInit {
                        key_code,
                        ..Default::default()
                    }
                    .init(),
                    to: vec![To::Mouse {
                        mouse_key: MouseKeyInit {
                            vertical_wheel: Some(vertical_wheel),
                            ..Default::default()
                        }
                        .init(),
                    }],
                    ..Default::default()
                }
                .init()
            })
            .collect(),
        },
        Rule {
            description: S("VK1+{1,2,3,4,5,6,7,8,9,0,-,^} -> F{1,2,3,4,5,6,7,8,9,10,11,12}"),
            manipulators: vec![
                (K::Key1, K::F1),
                (K::Key2, K::F2),
                (K::Key3, K::F3),
                (K::Key4, K::F4),
                (K::Key5, K::F5),
                (K::Key6, K::F6),
                (K::Key7, K::F7),
                (K::Key8, K::F8),
                (K::Key9, K::F9),
                (K::Key0, K::F10),
                (K::Hyphen, K::F11),
                (K::EqualSign, K::F12),
            ]
            .into_iter()
            .map(|(from, to)| {
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    from,
                    Some(FromModifier::Optional(vec![Any])),
                    to,
                    None,
                )
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+B -> Ctrl+Opt+Cmd+Shift+M (Maximize window size with ShiftIt)"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::B,
                    None,
                    K::M,
                    Some(vec![Ctrl, Opt, Cmd, Shift]),
                ),
            ],
        },
        Rule {
            description: S("VK1+Backslash -> Cmd+Opt+D (Hide the Dock)"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk1,
                    K::International3,
                    None,
                    K::D,
                    Some(vec![Cmd, Opt]),
                ),
            ],
        },
    ]
}

fn rules_vk2() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK2+F -> Cmd+Tab / VK2+D -> Cmd+Shift+Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::F,
                    None,
                    K::Tab,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::D,
                    None,
                    K::Tab,
                    Some(vec![Cmd, Shift]),
                ),
            ],
        },
        Rule {
            description: S("VK2+S -> Ctrl+Tab / VK2+A -> Ctrl+Shift+Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::S,
                    None,
                    K::Tab,
                    Some(vec![Ctrl]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::A,
                    None,
                    K::Tab,
                    Some(vec![Ctrl, Shift]),
                ),
            ],
        },
        Rule {
            description: S("VK2+9 -> Cmd+KeypadPlus / VK2+0 -> Cmd+Hyphen"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key9,
                    None,
                    K::KeypadPlus,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key0,
                    None,
                    K::Hyphen,
                    Some(vec![Cmd]),
                ),
            ],
        },
        Rule {
            description: S("VK2+1 -> VolumeDecrement / VK2+2 -> VolumeIncrement"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key1,
                    None,
                    K::VolumeDecrement,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key2,
                    None,
                    K::VolumeIncrement,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK2+3 -> BrightnessDecrement / VK2+4 -> BrightnessIncrement"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key3,
                    None,
                    K::DisplayBrightnessDecrement,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key4,
                    None,
                    K::DisplayBrightnessIncrement,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK2+Ctrl+{H,O,N,P,U,I,M,Comma,Period} -> Cmd+Ctrl+Opt+Shift+{Left,Right,Down,Up,1,2,3,4,N} (ShiftIt)"),
            manipulators: vec![
                (K::H, K::LeftArrow),
                (K::O, K::RightArrow),
                (K::N, K::DownArrow),
                (K::P, K::UpArrow),
                (K::U, K::Key1),
                (K::I, K::Key2),
                (K::M, K::Key3),
                (K::Comma, K::Key4),
                (K::J, K::P),
                (K::K, K::N),
            ]
                .into_iter()
                .map(|(from, to)| {
                    Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VK::Vk2,
                        from,
                        Some(FromModifier::Mandatory(vec![Ctrl])),
                        to,
                        Some(vec![Cmd, Ctrl, Opt, Shift]),
                    )
                })
                .collect_vec(),
        },
    ]
}

fn rules_open_apps() -> Vec<Rule> {
    vec![Rule {
        description: S("Open apps"),
        manipulators: vec![
            // (K::A, "Ctrl+Shift+Tab"),
            (K::B, "open -a 'Bitwarden.app'"),
            (K::C, "open -a 'Notion Calendar.app'"),
            // (K::D, "Command+Shift+Tab"),
            (
                K::E,
                r#"osascript -e "tell application \"Alfred 5\" to search \"snip \"""#,
            ),
            // (K::F, "Command+Tab"),
            (K::G, "open -a 'Visual Studio Code.app'"),
            (K::H, "open -a 'Visual Studio Code.app'"),
            (K::I, "open -a 'CLion.app'"),
            (K::J, "open -a 'Google Chrome.app'"),
            (K::K, "open -a 'iTerm.app'"),
            (K::L, "open -a 'Alfred 5.app'"),
            (K::M, "open -a 'Dynalist.app'"),
            (K::N, "open -a 'Notion.app'"),
            (K::O, "open -a 'Finder.app'"),
            (K::P, "open -a '1Password.app'"),
            // (K::Q, None),
            (K::R, "open -a 'jetbrains client 2023.1 eap.app'"),
            // (K::S, "Ctrl+Tab"),
            (K::T, "open -a 'Visual Studio Code.app'"),
            (K::U, "open -a 'Microsoft To Do.app'"),
            (K::V, "open -a 'DeepL.app'"),
            (
                K::W,
                r#"osascript -e "tell application \"Alfred 5\" to search \"define $(pbpaste)\"""#,
            ),
            (
                K::X,
                r#"osascript -e "tell application \"Alfred 5\" to search \"snip codeblocks\"""#,
            ),
            (K::Y, "open -a 'Slack.app'"),
            (K::Z, "open -a 'LICEcap.app'"),
            // (K::ReturnOrEnter, None),
            // (K::Quote, None), // :
            // (K::NonUsPound, None), // ]
            (K::OpenBracket, "open -a 'Mail.app'"), // @
            // (K::CloseBracket, None), // [
            (K::Comma, "open -a 'System Settings.app'"),
            (K::Period, "open -a 'ChatGPT.app'"),
            (
                K::Slash,
                "open 'https://s2.kingtime.jp/independent/recorder2/personal/'",
            ),
            // (K::International1, None), // _
            // (K::NonUsPound, None),
            // (K::Backslash, None),
        ]
        .into_iter()
        .map(|(key_code, shell_command)| {
            ManipulatorInit {
                conditions: Some(vec![Condition::with_vk2()]),
                from: FromInit {
                    key_code,
                    ..Default::default()
                }
                .init(),
                to: vec![To::Command { shell_command }],
                ..Default::default()
            }
            .init()
        })
        .collect_vec(),
    }]
}

fn rules_vk3() -> Vec<Rule> {
    vec![Rule {
        description: S("VK3+{A,S,D,F,G,H,J,K,L,Semicolon,Quote} -> {1,2,3,4,5,6,7,8,9,0,-}"),
        manipulators: vec![
            (K::A, K::Key1),
            (K::S, K::Key2),
            (K::D, K::Key3),
            (K::F, K::Key4),
            (K::G, K::Key5),
            (K::H, K::Key6),
            (K::J, K::Key7),
            (K::K, K::Key8),
            (K::L, K::Key9),
            (K::Semicolon, K::Key0),
            (K::Quote, K::Hyphen),
        ]
        .into_iter()
        .map(|(from, to)| {
            Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                VK::Vk3,
                from,
                Some(FromModifier::Optional(vec![Any])),
                to,
                None,
            )
        })
        .collect_vec(),
    }]
}

fn rules_semicolon() -> Vec<Rule> {
    vec![Rule {
        description: S("Semicolon -> Enter"),
        manipulators: vec![
            Manipulator::new_for_key_to_key_mapping(
                K::Semicolon,
                Some(FromModifier::Mandatory(vec![Ctrl])),
                K::Semicolon,
                None,
            ),
            Manipulator::new_for_key_to_key_mapping(
                K::Semicolon,
                Some(FromModifier::Mandatory(vec![Cmd, Shift])),
                K::KeypadPlus,
                Some(vec![Cmd]),
            ),
            Manipulator::new_for_key_to_key_mapping(K::Semicolon, None, K::ReturnOrEnter, None),
        ],
    }]
}

fn rules_singlequote() -> Vec<Rule> {
    vec![Rule {
        description: S("Ctrl+Colon -> SingleQuote"),
        manipulators: vec![ManipulatorInit {
            from: From {
                key_code: K::Quote,
                modifiers: Some(FromModifier::Mandatory(vec![Ctrl])),
            },
            to: vec![To::Key {
                key_code: K::Key7,
                modifiers: Some(vec![Shift]),
            }],
            ..Default::default()
        }
        .init()],
    }]
}

fn rules_disable_capslock() -> Vec<Rule> {
    vec![Rule {
        description: S("Disable CapsLock"),
        manipulators: vec![ManipulatorInit {
            from: From {
                key_code: K::CapsLock,
                modifiers: Some(FromModifier::Optional(vec![Any])),
            },
            to: vec![To::Key {
                key_code: K::VkNone,
                modifiers: None,
            }],
            ..Default::default()
        }
        .init()],
    }]
}
