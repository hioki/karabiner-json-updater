use crate::config::bundle_identifier::BundleIdentifier;
use crate::config::condition::Condition;
use crate::config::from::{From, FromInit, FromModifier};
use crate::config::key_code::*;
use crate::config::manipulator::{Manipulator, ManipulatorInit, ToAfterKeyUp, ToIfAlone};
use crate::config::modifier_key::ModifierKey;
use crate::config::mouse_key::MouseKeyInit;
use crate::config::rule::Rule;
use crate::config::set_variable::SetVariable;
use crate::config::to::{PointingButton, To};
use crate::config::value::Value;
use crate::config::virtual_key::VirtualKey;
use itertools::Itertools;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Config {
    pub title: &'static str,
    pub rules: Vec<Rule>,
}

impl Config {
    pub fn my_config() -> Config {
        Config {
            title: "Personal rules",
            rules: vec![
                Rule {
                    description: "Virtual Keys",
                    manipulators: vec![
                        (KeyCode::Lang1, VirtualKey::Vk1, Some(KeyCode::JapaneseKana)),
                        (
                            KeyCode::International4,
                            VirtualKey::Vk1,
                            Some(KeyCode::JapaneseKana),
                        ),
                        (
                            KeyCode::Lang2,
                            VirtualKey::Vk2,
                            Some(KeyCode::JapaneseEisuu),
                        ),
                        (
                            KeyCode::International5,
                            VirtualKey::Vk2,
                            Some(KeyCode::JapaneseEisuu),
                        ),
                        (KeyCode::RightGui, VirtualKey::Vk3, None),
                        (KeyCode::International2, VirtualKey::Vk3, None),
                        (KeyCode::Tab, VirtualKey::Vk4, Some(KeyCode::Tab)),
                    ]
                        .into_iter()
                        .map(|(key_code, virtual_key, to_if_alone)| ManipulatorInit {
                            from: From {
                                key_code,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
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
                        }.init())
                        .collect_vec(),
                },
                Rule {
                    description: "Substitute TMUX prefix with VK4 on iTerm2",
                    manipulators: vec![
                        KeyCode::C,
                        KeyCode::V,
                        KeyCode::H,
                        KeyCode::J,
                        KeyCode::K,
                        KeyCode::L,
                        KeyCode::N,
                        KeyCode::P,
                    ]
                        .into_iter()
                        .map(|key_code| ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: key_code.clone(),
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::new_tmux_prefix_key(),
                                To::Key {
                                    key_code,
                                    modifiers: Some(vec![ModifierKey::Ctrl]),
                                },
                            ],
                            ..Default::default()
                        }.init())
                        .collect_vec(),
                },
                Rule {
                    description: "VK4 on VSCode",
                    manipulators: vec![
                        KeyCode::Key1,
                        KeyCode::Key2,
                        KeyCode::Key3,
                        KeyCode::Key4,
                        KeyCode::A,
                        KeyCode::H,
                        KeyCode::J,
                        KeyCode::E,
                        KeyCode::L,
                        KeyCode::S,
                        KeyCode::P,
                        KeyCode::O,
                        KeyCode::C,
                        KeyCode::M,
                        KeyCode::K,
                        KeyCode::R,
                        KeyCode::X,
                        KeyCode::I,
                        KeyCode::Y,
                        KeyCode::CloseBracket,
                        KeyCode::NonUsPound,
                    ]
                        .into_iter()
                        .map(|key_code| ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::VSCode),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: key_code.clone(),
                                ..Default::default()
                            }.init(),
                            to: vec![To::Key {
                                key_code,
                                modifiers: Some(vec![
                                    ModifierKey::Ctrl,
                                    ModifierKey::Shift,
                                    ModifierKey::Opt,
                                    ModifierKey::Cmd,
                                ]),
                            }],
                            ..Default::default()
                        }.init())
                        .collect_vec(),
                },
                Rule {
                    description: "[CLion] ¥ -> \\",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::International3,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::International3,
                                    modifiers: Some(vec![ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+E -> Cmd+1 (Project)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::E,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Key1,
                                    modifiers: Some(vec![ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+F -> Cmd+Shift+O (Files)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::F,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::O,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+K -> Cmd+Shift+F (Find in Path)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::K,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+N -> F2 (Next Problems)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::N,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F2,
                                    modifiers: None,
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+P -> Shift+F2 (Previous Problems)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::P,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F2,
                                    modifiers: Some(vec![ModifierKey::Shift]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+; -> Opt+Enter (More Actions)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Semicolon,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::ReturnOrEnter,
                                    modifiers: Some(vec![ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+R -> Opt+Cmd+Y (File Reload)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::R,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Y,
                                    modifiers: Some(vec![ModifierKey::Opt, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+T -> Opt+Cmd+B (Type)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::T,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::B,
                                    modifiers: Some(vec![ModifierKey::Opt, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+S -> Opt+Cmd+O (Symbols)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::S,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::O,
                                    modifiers: Some(vec![ModifierKey::Opt, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+C -> Cmd+O (Classes)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::C,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::O,
                                    modifiers: Some(vec![ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+A -> Cmd+Shift+A (Actions)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::A,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::A,
                                    modifiers: Some(vec![ModifierKey::Shift, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+H -> Ctrl+Opt+UpArrow",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::H,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::UpArrow,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+L -> Ctrl+Opt+DownArrow",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::L,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::DownArrow,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+[ -> Opt+Cmd+[",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::CloseBracket,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Shift])),
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::CloseBracket,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+] -> Opt+Cmd+]",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Backslash,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Shift])),
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Backslash,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+M -> F3 (Bookmark)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::M,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F3,
                                    modifiers: None,
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+, -> Ctrl+Shift+Opt+, (Previous Bookmark)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Comma,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Comma,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Shift, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+. -> Ctrl+Shift+Opt+. (Next Bookmark)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Period,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Period,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Shift, ModifierKey::Opt]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+B -> Cmd+F3 (Show Bookmarks)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::B,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F3,
                                    modifiers: Some(vec![ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+O -> Ctrl+Shift+Opt+Cmd+O (Open Recent)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::O,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::O,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Shift, ModifierKey::Opt, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+J -> Cmd+S (Save All)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::J,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::S,
                                    modifiers: Some(vec![ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+@ -> F1 (Quick Documentation)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::OpenBracket,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Shift])),
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F1,
                                    modifiers: None,
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+/ -> Ctrl+Shift+Opt+Cmd+S (Search Everywhere)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Slash,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::S,
                                    modifiers: Some(vec![ModifierKey::Ctrl, ModifierKey::Shift, ModifierKey::Opt, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[CLion] VK4+: -> Cmd+Shift+E (Recent Locations)",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::Quote,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::E,
                                    modifiers: Some(vec![ModifierKey::Shift, ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                // CLion unassigned: U I
                Rule {
                    description: "[Dynalist] VK1+U/I -> Ctrl+A/E",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Dynalist),
                                Condition::with_vk1(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::U,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::A,
                                    modifiers: Some(vec![ModifierKey::Ctrl]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Dynalist),
                                Condition::with_vk1(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::I,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::E,
                                    modifiers: Some(vec![ModifierKey::Ctrl]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[Atom] VK4+F -> Cmd+T",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Atom),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::F,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::T,
                                    modifiers: Some(vec![ModifierKey::Cmd]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[Atom] VK4+E -> Ctrl+O",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Atom),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::E,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::Key0,
                                    modifiers: Some(vec![ModifierKey::Ctrl]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[Atom] VK4+K -> Cmd+Shift+F",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Atom),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::K,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::F,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[Atom] VK4+A -> Cmd+Shift+P",
                    manipulators: vec![
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::Atom),
                                Condition::with_vk4(),
                            ]),
                            from: FromInit {
                                key_code: KeyCode::A,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::P,
                                    modifiers: Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                                },
                            ],
                            ..Default::default()
                        }.init(),
                    ]
                },
                Rule {
                    description: "[iTerm2] VK1+O -> Ctrl+T Ctrl+P / VK1+P -> Ctrl+T Ctrl+N",
                    manipulators: vec![
                        (KeyCode::O, KeyCode::P),
                        (KeyCode::P, KeyCode::N),
                    ].into_iter().map(|(from, to)| {
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_vk1(),
                            ]),
                            from: FromInit {
                                key_code: from,
                                ..Default::default()
                            }.init(),
                            to: vec![
                                To::new_tmux_prefix_key(),
                                To::Key {
                                    key_code: to,
                                    modifiers: Some(vec![ModifierKey::Ctrl]),
                                },
                            ],
                            ..Default::default()
                        }.init()
                    }).collect_vec(),
                },
                Rule {
                    description: "[iTerm2] VK2+H -> Backspace",
                    manipulators: vec![ManipulatorInit {
                        conditions: Some(vec![Condition::with_vk2()]),
                        from: FromInit {
                            key_code: KeyCode::H,
                            ..Default::default()
                        }.init(),
                        to: vec![To::Key {
                            key_code: KeyCode::DeleteOrBackspace,
                            modifiers: None,
                        }],
                        ..Default::default()
                    }.init()],
                },
                Rule {
                    description: "[iTerm2] VK1+Z -> Enter tmux copy-mode",
                    manipulators: vec![ManipulatorInit {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_vk1(),
                        ]),
                        from: FromInit {
                            key_code: KeyCode::Z,
                            ..Default::default()
                        }.init(),
                        to: vec![
                            To::new_tmux_prefix_key(),
                            To::Key {
                                key_code: KeyCode::CloseBracket,
                                modifiers: Some(vec![ModifierKey::Ctrl]),
                            },
                        ],
                        ..Default::default()
                    }.init()],
                },
                Rule {
                    description: "[iTerm2] VK1+U -> Shift+0 / VK1+I -> shift+4",
                    manipulators: vec![
                        (KeyCode::U, KeyCode::Key0),
                        (KeyCode::I, KeyCode::Key4),
                    ].into_iter().map(|(from, to)| {
                        ManipulatorInit {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_vk1(),
                            ]),
                            from: FromInit {
                                key_code: from,
                                ..Default::default()
                            }.init(),
                            to: vec![To::Key {
                                key_code: to,
                                modifiers: Some(vec![ModifierKey::Shift]),
                            }],
                            ..Default::default()
                        }.init()
                    }).collect_vec()
                },
                Rule {
                    description: "VK1+{H/J/K/L} -> {Left/Down/Up/Right}Arrow",
                    manipulators: vec![
                        (KeyCode::H, KeyCode::LeftArrow),
                        (KeyCode::J, KeyCode::DownArrow),
                        (KeyCode::K, KeyCode::UpArrow),
                        (KeyCode::L, KeyCode::RightArrow),
                    ]
                        .into_iter()
                        .map(|(from, to)| {
                            Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                                VirtualKey::Vk1,
                                from,
                                Some(FromModifier::Optional(vec![ModifierKey::Any])),
                                to,
                                None,
                            )
                        })
                        .collect_vec(),
                },
                Rule {
                    description: "VK1+F -> Escape",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::F,
                        Some(FromModifier::Optional(vec![ModifierKey::Any])),
                        KeyCode::Escape,
                        None,
                    )],
                },
                Rule {
                    description: "VK1+S -> Shift+Ctrl+J / VK1+D -> Shift+Ctrl+Semicolon (IME Switching of Google Japanese Input)",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::S,
                            None,
                            KeyCode::J,
                            Some(vec![ModifierKey::Shift, ModifierKey::Ctrl]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::D,
                            None,
                            KeyCode::Semicolon,
                            Some(vec![ModifierKey::Shift, ModifierKey::Ctrl]),
                        ),
                    ],
                },
                Rule {
                    description: "VK1+A -> F10 / VK1+Z -> F7",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(VirtualKey::Vk1, KeyCode::A, None, KeyCode::F10, None),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(VirtualKey::Vk1, KeyCode::Z, None, KeyCode::F7, None),
                    ],
                },
                Rule {
                    description: "VK1+U -> Cmd+LeftArrow / VK1+I -> Cmd+RightArrow",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::U,
                            Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            KeyCode::LeftArrow,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::I,
                            Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            KeyCode::RightArrow,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                    ],
                },
                Rule {
                    description: "VK1+G -> Tab",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::G,
                        Some(FromModifier::Optional(vec![ModifierKey::Any])),
                        KeyCode::Tab,
                        None,
                    )],
                },
                Rule {
                    description: "VK1+O -> Ctrl+Shift+Tab / VK1+P -> Ctrl+Tab",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::O,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Ctrl, ModifierKey::Shift]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::P,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Ctrl]),
                        ),
                    ],
                },
                Rule {
                    description: "VK1+Y -> Cmd+C / VK1+T -> Cmd+X / VK1+X -> Cmd+Shift+V",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::Y,
                            None,
                            KeyCode::C,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::T,
                            None,
                            KeyCode::X,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::X,
                            None,
                            KeyCode::V,
                            Some(vec![
                                ModifierKey::Cmd,
                                ModifierKey::Shift,
                                ModifierKey::Opt,
                            ]),
                        ),
                    ],
                },
                Rule {
                    description: "VK1+C -> Backspace / VK1+E -> Delete",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::C,
                            None,
                            KeyCode::DeleteOrBackspace,
                            None,
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::E,
                            None,
                            KeyCode::DeleteForward,
                            None,
                        ),
                    ],
                },
                Rule {
                    description: "VK1+[ -> Cmd+Z",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::CloseBracket,
                        Some(FromModifier::Optional(vec![ModifierKey::Any])),
                        KeyCode::Z,
                        Some(vec![ModifierKey::Cmd]),
                    )],
                },
                Rule {
                    description: "VK1+Colon -> Cmd+H",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::Quote,
                        None,
                        KeyCode::H,
                        Some(vec![ModifierKey::Cmd]),
                    )],
                },
                Rule {
                    description: "VK1+{N,M,Comma,Period} -> Mouse{Left,Down,Up,Right}",
                    manipulators: vec![
                        (
                            KeyCode::N,
                            Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                            Some(-1536),
                            None,
                        ),
                        (
                            KeyCode::M,
                            Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                            None,
                            Some(1536),
                        ),
                        (
                            KeyCode::Comma,
                            Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                            None,
                            Some(-1536),
                        ),
                        (
                            KeyCode::Period,
                            Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                            Some(1536),
                            None,
                        ),
                        (KeyCode::N, None, Some(-3072), None),
                        (KeyCode::M, None, None, Some(3072)),
                        (KeyCode::Comma, None, None, Some(-3072)),
                        (KeyCode::Period, None, Some(3072), None),
                    ]
                        .into_iter()
                        .map(|(key_code, modifiers, x, y)| ManipulatorInit {
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
                                }.init(),
                            }],
                            ..Default::default()
                        }.init())
                        .collect_vec(),
                },
                Rule {
                    description: "VK1+Slash -> LeftClick / VK1+Underscore -> RightClick",
                    manipulators: vec![
                        (KeyCode::Slash, PointingButton::Button1),
                        (KeyCode::International1, PointingButton::Button2),
                    ].into_iter().map(|(from, to)|{
                        ManipulatorInit {
                            conditions: Some(vec![Condition::with_vk1()]),
                            from: From {
                                key_code: from,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            },
                            to: vec![To::Click {
                                pointing_button: to,
                            }],
                            ..Default::default()
                        }.init()
                    }).collect_vec(),
                },
                Rule {
                    description: "VK1+@ -> ScrollUp / VK1+] -> ScrollDown",
                    manipulators: vec![
                        (KeyCode::OpenBracket, -64),
                        (KeyCode::NonUsPound, 64),
                        (KeyCode::Backslash, 64),
                    ]
                        .into_iter()
                        .map(|(key_code, vertical_wheel)| ManipulatorInit {
                            conditions: Some(vec![Condition::with_vk1()]),
                            from: FromInit {
                                key_code,
                                ..Default::default()
                            }.init(),
                            to: vec![To::Mouse {
                                mouse_key: MouseKeyInit {
                                    vertical_wheel: Some(vertical_wheel),
                                    ..Default::default()
                                }.init(),
                            }],
                            ..Default::default()
                        }.init())
                        .collect(),
                },
                Rule {
                    description: "VK1+{1,2,3,4,5,6,7,8,9,0,-,^} -> F{1,2,3,4,5,6,7,8,9,10,11,12}",
                    manipulators: vec![
                        (KeyCode::Key1, KeyCode::F1),
                        (KeyCode::Key2, KeyCode::F2),
                        (KeyCode::Key3, KeyCode::F3),
                        (KeyCode::Key4, KeyCode::F4),
                        (KeyCode::Key5, KeyCode::F5),
                        (KeyCode::Key6, KeyCode::F6),
                        (KeyCode::Key7, KeyCode::F7),
                        (KeyCode::Key8, KeyCode::F8),
                        (KeyCode::Key9, KeyCode::F9),
                        (KeyCode::Key0, KeyCode::F10),
                        (KeyCode::Hyphen, KeyCode::F11),
                        (KeyCode::EqualSign, KeyCode::F12),
                    ]
                        .into_iter()
                        .map(|(from, to)| Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(VirtualKey::Vk1, from, Some(FromModifier::Optional(vec![ModifierKey::Any])), to, None))
                        .collect_vec(),
                },
                Rule {
                    description: "VK1+B -> Ctrl+Opt+Cmd+M (Maximize window size with ShiftIt)",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::B,
                        None,
                        KeyCode::M,
                        Some(vec![
                            ModifierKey::Ctrl,
                            ModifierKey::Opt,
                            ModifierKey::Cmd,
                        ]),
                    )],
                },
                Rule {
                    description: "VK1+Backslash -> Cmd+Opt+D (Hide the Dock)",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::International3,
                        None,
                        KeyCode::D,
                        Some(vec![ModifierKey::Cmd, ModifierKey::Opt]),
                    )],
                },
                Rule {
                    description: "VK2+F -> Cmd+Tab / VK2+D -> Cmd+Shift+Tab",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::F,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::D,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                        ),
                    ],
                },
                Rule {
                    description: "VK2+S -> Ctrl+Tab / VK2+A -> Ctrl+Shift+Tab",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::S,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Ctrl]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::A,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Ctrl, ModifierKey::Shift]),
                        ),
                    ],
                },
                Rule {
                    description: "VK2+9 -> Cmd+Shift+Semicolon / VK2+0 -> Cmd+Hyphen",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key9,
                            None,
                            KeyCode::Semicolon,
                            Some(vec![ModifierKey::Cmd, ModifierKey::Shift]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key0,
                            None,
                            KeyCode::Hyphen,
                            Some(vec![ModifierKey::Cmd]),
                        ),
                    ],
                },
                Rule {
                    description: "VK2+1 -> VolumeDecrement / VK2+2 -> VolumeIncrement",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key1,
                            None,
                            KeyCode::VolumeDecrement,
                            None,
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key2,
                            None,
                            KeyCode::VolumeIncrement,
                            None,
                        ),
                    ],
                },
                Rule {
                    description: "VK2+3 -> BrightnessDecrement / VK2+4 -> BrightnessIncrement",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key3,
                            None,
                            KeyCode::DisplayBrightnessDecrement,
                            None,
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key4,
                            None,
                            KeyCode::DisplayBrightnessIncrement,
                            None,
                        ),
                    ],
                },
                Rule {
                    description: "VK2+Ctrl+{H,O,N,P,U,I,M,Comma} -> Cmd+Ctrl+Opt+{Left,Right,Down,Up,1,2,3,4} (ShiftIt)",
                    manipulators: vec![
                        (KeyCode::H, KeyCode::LeftArrow),
                        (KeyCode::O, KeyCode::RightArrow),
                        (KeyCode::N, KeyCode::DownArrow),
                        (KeyCode::P, KeyCode::UpArrow),
                        (KeyCode::U, KeyCode::Key1),
                        (KeyCode::I, KeyCode::Key2),
                        (KeyCode::M, KeyCode::Key3),
                        (KeyCode::Comma, KeyCode::Key4),
                    ]
                        .into_iter()
                        .map(|(from, to)| {
                            Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                                VirtualKey::Vk2,
                                from,
                                Some(FromModifier::Mandatory(vec![ModifierKey::Ctrl])),
                                to,
                                Some(vec![
                                    ModifierKey::Cmd,
                                    ModifierKey::Ctrl,
                                    ModifierKey::Opt,
                                ]),
                            )
                        })
                        .collect_vec(),
                },
                Rule {
                    description: "VK2+C -> Dictionary",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::C,
                            None,
                            KeyCode::D,
                            Some(vec![ModifierKey::Cmd, ModifierKey::Ctrl])
                        )
                    ]
                },
                Rule {
                    description: "VK2+Z -> Ctrl+Shift+Cmd+Opt+T (Launch alfred-google-translate-formatter-workflow)", // https://github.com/pddg/alfred-google-translate-formatter-workflow
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Z,
                            None,
                            KeyCode::T,
                            Some(vec![ModifierKey::Ctrl, ModifierKey::Shift, ModifierKey::Cmd, ModifierKey::Opt])
                        )
                    ]
                },
                Rule {
                    description: "Open apps",
                    manipulators: vec![
                        (KeyCode::B, "open -a 'Tweetbot.app'"),
                        (
                            KeyCode::E,
                            r#"osascript -e "tell application \"Alfred 4\" to search \"snip \"""#,
                        ),
                        (KeyCode::G, "open -a 'Atom.app'"),
                        (KeyCode::I, "open -a 'CLion.app'"),
                        (KeyCode::J, "open -a 'Google Chrome.app'"),
                        (KeyCode::K, "open -a 'iTerm.app'"),
                        (KeyCode::L, "open -a 'Alfred 4.app'"),
                        (KeyCode::M, "open -a 'Dynalist.app'"),
                        (KeyCode::N, "open -a 'Notes.app'"),
                        (KeyCode::P, "open -a '1Password.app'"),
                        (
                            KeyCode::R,
                            r#"osascript -e "tell application \"Alfred 4\" to search \"docsrs \"""#,
                        ),
                        (KeyCode::T, "open -a 'Microsoft To Do.app'"),
                        (KeyCode::V, "open -a 'Visual Studio Code.app'"),
                        (
                            KeyCode::X,
                            r#"osascript -e "tell application \"Alfred 4\" to search \"snip codeblocks\"""#,
                        ),
                        (KeyCode::Y, "open -a 'Spotify.app'"),
                        (KeyCode::Comma, "open -a 'System Preferences.app'"),
                        (KeyCode::Slash, "open -a 'Slack.app'"),
                        (KeyCode::OpenBracket, "open -a 'Mail.app'"),
                    ]
                        .into_iter()
                        .map(|(key_code, shell_command)| ManipulatorInit {
                            conditions: Some(vec![Condition::with_vk2()]),
                            from: FromInit {
                                key_code,
                                ..Default::default()
                            }.init(),
                            to: vec![To::Command { shell_command }],
                            ..Default::default()
                        }.init())
                        .collect_vec(),
                },
                Rule {
                    description: "VK3+{A,S,D,F,G,H,J,K,L,Semicolon,Quote} -> {1,2,3,4,5,6,7,8,9,0,-}",
                    manipulators: vec![
                        (KeyCode::A, KeyCode::Key1),
                        (KeyCode::S, KeyCode::Key2),
                        (KeyCode::D, KeyCode::Key3),
                        (KeyCode::F, KeyCode::Key4),
                        (KeyCode::G, KeyCode::Key5),
                        (KeyCode::H, KeyCode::Key6),
                        (KeyCode::J, KeyCode::Key7),
                        (KeyCode::K, KeyCode::Key8),
                        (KeyCode::L, KeyCode::Key9),
                        (KeyCode::Semicolon, KeyCode::Key0),
                        (KeyCode::Quote, KeyCode::Hyphen),
                    ]
                        .into_iter()
                        .map(|(from, to)| {
                            Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                                VirtualKey::Vk3,
                                from,
                                Some(FromModifier::Optional(vec![ModifierKey::Any])),
                                to,
                                None,
                            )
                        })
                        .collect_vec(),
                },
                Rule {
                    description: "Semicolon -> Enter",
                    manipulators: vec![
                        Manipulator::new_for_key_to_key_mapping(KeyCode::Semicolon, Some(FromModifier::Mandatory(vec![ModifierKey::Ctrl])), KeyCode::Semicolon, None),
                        Manipulator::new_for_key_to_key_mapping(KeyCode::Semicolon, Some(FromModifier::Mandatory(vec![ModifierKey::Cmd, ModifierKey::Shift])), KeyCode::KeypadPlus, Some(vec![ModifierKey::Cmd])),
                        Manipulator::new_for_key_to_key_mapping(KeyCode::Semicolon, None, KeyCode::ReturnOrEnter, None),
                    ],
                },
                Rule {
                    description: "Ctrl+Colon -> SingleQuote",
                    manipulators: vec![ManipulatorInit {
                        from: From {
                            key_code: KeyCode::Quote,
                            modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Ctrl])),
                        },
                        to: vec![To::Key {
                            key_code: KeyCode::Key7,
                            modifiers: Some(vec![ModifierKey::Shift]),
                        }],
                        ..Default::default()
                    }.init()],
                },
                Rule {
                    description: "Disable CapsLock",
                    manipulators: vec![ManipulatorInit {
                        from: From {
                            key_code: KeyCode::CapsLock,
                            modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                        },
                        to: vec![To::Key {
                            key_code: KeyCode::VkNone,
                            modifiers: None,
                        }],
                        ..Default::default()
                    }.init()],
                },
            ],
        }
    }
}
