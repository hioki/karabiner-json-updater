use crate::config::bundle_identifier::BundleIdentifier;
use crate::config::condition::Condition;
use crate::config::from::{From, FromModifier};
use crate::config::key_code::*;
use crate::config::manipulator::{Manipulator, ToAfterKeyUp, ToIfAlone};
use crate::config::modifier_key::ModifierKey;
use crate::config::mouse_key::MouseKey;
use crate::config::rule::Rule;
use crate::config::set_variable::SetVariable;
use crate::config::to::{PointingButton, To};
use crate::config::value::Value;
use crate::config::virtual_key::VirtualKey;

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
                        .map(|(key_code, virtual_key, to_if_alone)| Manipulator {
                            r#type: Default::default(),
                            conditions: None,
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
                        })
                        .collect::<Vec<Manipulator>>(),
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
                        .map(|key_code| Manipulator {
                            r#type: Default::default(),
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_virtual_key(VirtualKey::Vk4),
                            ]),
                            from: From {
                                key_code: key_code.clone(),
                                modifiers: None,
                            },
                            to: vec![
                                To::new_tmux_prefix_key(),
                                To::Key {
                                    key_code,
                                    modifiers: Some(vec![ModifierKey::Control]),
                                },
                            ],
                            to_after_key_up: None,
                            to_if_alone: None,
                        })
                        .collect::<Vec<Manipulator>>(),
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
                        .map(|key_code| Manipulator {
                            r#type: Default::default(),
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::VSCode),
                                Condition::with_virtual_key(VirtualKey::Vk4),
                            ]),
                            from: From {
                                key_code: key_code.clone(),
                                modifiers: None,
                            },
                            to: vec![To::Key {
                                key_code,
                                modifiers: Some(vec![
                                    ModifierKey::Control,
                                    ModifierKey::Shift,
                                    ModifierKey::Option,
                                    ModifierKey::Command,
                                ]),
                            }],
                            to_after_key_up: None,
                            to_if_alone: None,
                        })
                        .collect::<Vec<Manipulator>>(),
                },
                Rule {
                    description: "[CLion] ¥ -> \\",
                    manipulators: vec![
                        Manipulator {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::CLion),
                            ]),
                            from: From {
                                key_code: KeyCode::International3,
                                modifiers: None,
                            },
                            to: vec![
                                To::Key {
                                    key_code: KeyCode::International3,
                                    modifiers: Some(vec![ModifierKey::Option]),
                                },
                            ],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                    ]
                },
                Rule {
                    description: "[iTerm2] VK1+O -> Ctrl+T Ctrl+P / VK1+P -> Ctrl+T Ctrl+N",
                    manipulators: vec![
                        Manipulator {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_virtual_key(VirtualKey::Vk1),
                            ]),
                            from: From {
                                key_code: KeyCode::O,
                                modifiers: None,
                            },
                            to: vec![
                                To::new_tmux_prefix_key(),
                                To::Key {
                                    key_code: KeyCode::P,
                                    modifiers: Some(vec![ModifierKey::Control]),
                                },
                            ],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                        Manipulator {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_virtual_key(VirtualKey::Vk1),
                            ]),
                            from: From {
                                key_code: KeyCode::P,
                                modifiers: None,
                            },
                            to: vec![
                                To::new_tmux_prefix_key(),
                                To::Key {
                                    key_code: KeyCode::N,
                                    modifiers: Some(vec![ModifierKey::Control]),
                                },
                            ],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                    ],
                },
                Rule {
                    description: "[iTerm2] VK2+H -> Backspace",
                    manipulators: vec![Manipulator {
                        conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                        from: From {
                            key_code: KeyCode::H,
                            modifiers: None,
                        },
                        to: vec![To::Key {
                            key_code: KeyCode::DeleteOrBackspace,
                            modifiers: None,
                        }],
                        r#type: Default::default(),
                        to_after_key_up: None,
                        to_if_alone: None,
                    }],
                },
                Rule {
                    description: "[iTerm2] VK1+Z -> Enter tmux copy-mode",
                    manipulators: vec![Manipulator {
                        conditions: Some(vec![
                            Condition::on_app(BundleIdentifier::ITerm2),
                            Condition::with_virtual_key(VirtualKey::Vk1),
                        ]),
                        from: From {
                            key_code: KeyCode::Z,
                            modifiers: None,
                        },
                        to: vec![
                            To::new_tmux_prefix_key(),
                            To::Key {
                                key_code: KeyCode::CloseBracket,
                                modifiers: Some(vec![ModifierKey::Control]),
                            },
                        ],
                        r#type: Default::default(),
                        to_after_key_up: None,
                        to_if_alone: None,
                    }],
                },
                Rule {
                    description: "[iTerm2] VK1+U -> Shift+0 / VK1+I -> shift+4",
                    manipulators: vec![
                        Manipulator {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_virtual_key(VirtualKey::Vk1),
                            ]),
                            from: From {
                                key_code: KeyCode::U,
                                modifiers: None,
                            },
                            to: vec![To::Key {
                                key_code: KeyCode::Key0,
                                modifiers: Some(vec![ModifierKey::Shift]),
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                        Manipulator {
                            conditions: Some(vec![
                                Condition::on_app(BundleIdentifier::ITerm2),
                                Condition::with_virtual_key(VirtualKey::Vk1),
                            ]),
                            from: From {
                                key_code: KeyCode::I,
                                modifiers: None,
                            },
                            to: vec![To::Key {
                                key_code: KeyCode::Key4,
                                modifiers: Some(vec![ModifierKey::Shift]),
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                    ],
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
                        .collect::<Vec<Manipulator>>(),
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
                            Some(vec![ModifierKey::Shift, ModifierKey::Control]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::D,
                            None,
                            KeyCode::Semicolon,
                            Some(vec![ModifierKey::Shift, ModifierKey::Control]),
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
                            Some(vec![ModifierKey::Command]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::I,
                            Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            KeyCode::RightArrow,
                            Some(vec![ModifierKey::Command]),
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
                            Some(vec![ModifierKey::Control, ModifierKey::Shift]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::P,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Control]),
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
                            Some(vec![ModifierKey::Command]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::T,
                            None,
                            KeyCode::X,
                            Some(vec![ModifierKey::Command]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk1,
                            KeyCode::X,
                            None,
                            KeyCode::V,
                            Some(vec![
                                ModifierKey::Command,
                                ModifierKey::Shift,
                                ModifierKey::Option,
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
                        Some(vec![ModifierKey::Command]),
                    )],
                },
                Rule {
                    description: "VK1+Colon -> Cmd+H",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::Quote,
                        None,
                        KeyCode::H,
                        Some(vec![ModifierKey::Command]),
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
                        .map(|(key_code, modifiers, x, y)| Manipulator {
                            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                            from: From {
                                key_code,
                                modifiers,
                            },
                            to: vec![To::Mouse {
                                mouse_key: MouseKey {
                                    x,
                                    y,
                                    vertical_wheel: None,
                                },
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        })
                        .collect::<Vec<Manipulator>>(),
                },
                Rule {
                    description: "VK1+Slash -> LeftClick / VK1+Underscore -> RightClick",
                    manipulators: vec![
                        Manipulator {
                            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                            from: From {
                                key_code: KeyCode::Slash,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            },
                            to: vec![To::Click {
                                pointing_button: PointingButton::Button1,
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                        Manipulator {
                            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                            from: From {
                                key_code: KeyCode::International1,
                                modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                            },
                            to: vec![To::Click {
                                pointing_button: PointingButton::Button2,
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                    ],
                },
                Rule {
                    description: "VK1+@ -> ScrollUp / VK1+] -> ScrollDown",
                    manipulators: vec![
                        (KeyCode::OpenBracket, -64),
                        (KeyCode::NonUsPound, 64),
                        (KeyCode::Backslash, 64),
                    ]
                        .into_iter()
                        .map(|(key_code, vertical_wheel)| Manipulator {
                            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                            from: From {
                                key_code,
                                modifiers: None,
                            },
                            to: vec![To::Mouse {
                                mouse_key: MouseKey {
                                    x: None,
                                    y: None,
                                    vertical_wheel: Some(vertical_wheel),
                                },
                            }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        })
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
                        .collect::<Vec<Manipulator>>(),
                },
                Rule {
                    description: "VK1+B -> Ctrl+Opt+Cmd+M (Maximize window size with ShiftIt)",
                    manipulators: vec![Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VirtualKey::Vk1,
                        KeyCode::B,
                        None,
                        KeyCode::M,
                        Some(vec![
                            ModifierKey::Control,
                            ModifierKey::Option,
                            ModifierKey::Command,
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
                        Some(vec![ModifierKey::Command, ModifierKey::Option]),
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
                            Some(vec![ModifierKey::Command]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::D,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Command, ModifierKey::Shift]),
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
                            Some(vec![ModifierKey::Control]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::A,
                            None,
                            KeyCode::Tab,
                            Some(vec![ModifierKey::Control, ModifierKey::Shift]),
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
                            Some(vec![ModifierKey::Command, ModifierKey::Shift]),
                        ),
                        Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                            VirtualKey::Vk2,
                            KeyCode::Key0,
                            None,
                            KeyCode::Hyphen,
                            Some(vec![ModifierKey::Command]),
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
                    description: "VK2+{H,O,N,P,U,I,M,Comma} -> Cmd+Ctrl+Opt+{Left,Right,Down,Up,1,2,3,4} (ShiftIt)",
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
                                Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                                to,
                                Some(vec![
                                    ModifierKey::Command,
                                    ModifierKey::Control,
                                    ModifierKey::Option,
                                ]),
                            )
                        })
                        .collect::<Vec<Manipulator>>(),
                },
                Rule {
                    description: "Open apps",
                    manipulators: vec![
                        (KeyCode::J, "open -a 'Google Chrome.app'"),
                        (KeyCode::L, "open -a 'Alfred 4.app'"),
                        (KeyCode::K, "open -a 'iTerm.app'"),
                        (KeyCode::L, "open -a 'Alfred 4.app'"),
                        (KeyCode::I, "open -a 'CLion.app'"),
                        (
                            KeyCode::E,
                            r#"osascript -e "tell application \"Alfred 4\" to search \"snip \"""#,
                        ),
                        (KeyCode::Slash, "open -a 'Slack.app'"),
                        (KeyCode::OpenBracket, "open -a 'Mail.app'"),
                        (KeyCode::T, "open -a 'Microsoft To Do.app'"),
                        (KeyCode::G, "open -a 'Atom.app'"),
                        (KeyCode::B, "open -a 'Tweetbot.app'"),
                        (KeyCode::M, "open -a 'Skim.app'"),
                        (KeyCode::R, "open -a 'Notes.app'"),
                        (KeyCode::V, "open -a 'Visual Studio Code.app'"),
                        (KeyCode::W, "open -a '1Password.app'"),
                    ]
                        .into_iter()
                        .map(|(key_code, shell_command)| Manipulator {
                            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                            from: From {
                                key_code,
                                modifiers: None,
                            },
                            to: vec![To::Command { shell_command }],
                            r#type: Default::default(),
                            to_after_key_up: None,
                            to_if_alone: None,
                        })
                        .collect::<Vec<Manipulator>>(),
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
                        .collect::<Vec<Manipulator>>(),
                },
                Rule {
                    description: "Semicolon -> Enter",
                    manipulators: vec![
                        Manipulator {
                            r#type: Default::default(),
                            conditions: None,
                            from: From {
                                key_code: KeyCode::Semicolon,
                                modifiers: None,
                            },
                            to: vec![To::Key {
                                key_code: KeyCode::ReturnOrEnter,
                                modifiers: None,
                            }],
                            to_after_key_up: None,
                            to_if_alone: None,
                        },
                    ],
                },
                Rule {
                    description: "Ctrl+Colon -> SingleQuote",
                    manipulators: vec![Manipulator {
                        r#type: Default::default(),
                        conditions: None,
                        from: From {
                            key_code: KeyCode::Quote,
                            modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                        },
                        to: vec![To::Key {
                            key_code: KeyCode::Key7,
                            modifiers: Some(vec![ModifierKey::Shift]),
                        }],
                        to_after_key_up: None,
                        to_if_alone: None,
                    }],
                },
                Rule {
                    description: "Disable CapsLock",
                    manipulators: vec![Manipulator {
                        r#type: Default::default(),
                        conditions: None,
                        from: From {
                            key_code: KeyCode::CapsLock,
                            modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                        },
                        to: vec![To::Key {
                            key_code: KeyCode::VkNone,
                            modifiers: None,
                        }],
                        to_after_key_up: None,
                        to_if_alone: None,
                    }],
                },
            ],
        }
    }
}
