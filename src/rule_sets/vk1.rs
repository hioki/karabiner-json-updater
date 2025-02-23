use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
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
            description: S("VK1+Shift+Y -> Cmd+C and remove all newlines"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![Condition::with_vk1()]),
                from: From {
                    key_code: K::Y,
                    modifiers: Some(FromModifier::Mandatory(vec![Shift])),
                },
                to: vec![
                    To::Key {
                        key_code: K::C,
                        modifiers: Some(vec![Cmd]),
                    },
                    To::Command {
                        shell_command: "export LC_ALL=en_US.UTF-8; pbpaste | tr -d '\n' | sed 's/  */ /g' | pbcopy",
                    },
                ],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("VK1+Y -> Cmd+C"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![Condition::with_vk1()]),
                from: From {
                    key_code: K::Y,
                    modifiers: None,
                },
                to: vec![To::Key {
                    key_code: K::C,
                    modifiers: Some(vec![Cmd]),
                }],
                ..Default::default()
            }
            .init()],
        },
        Rule {
            description: S("VK1+T -> Cmd+X, VK1+X -> Cmd+Shift+V"),
            manipulators: vec![
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
