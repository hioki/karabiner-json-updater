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
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                    .to_key(to, None)
                    .build()
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+F -> Escape"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key_with_modifiers(K::F, FromModifier::Optional(vec![Any]))
                .to_key(K::Escape, None)
                .build()],
        },
        Rule {
            description: S("VK1+S -> JapaneseKana / VK1+D -> JapaneseEisuu"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::S)
                    .to_key(K::JapaneseKana, None)
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::D)
                    .to_key(K::JapaneseEisuu, None)
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+A -> F10 / VK1+Z -> F7"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::A)
                    .to_key(K::F10, None)
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::Z)
                    .to_key(K::F7, None)
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+U -> Cmd+LeftArrow / VK1+I -> Cmd+RightArrow"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key_with_modifiers(K::U, FromModifier::Optional(vec![Any]))
                    .to_key(K::LeftArrow, Some(vec![Cmd]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key_with_modifiers(K::I, FromModifier::Optional(vec![Any]))
                    .to_key(K::RightArrow, Some(vec![Cmd]))
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+G -> Tab"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key_with_modifiers(K::G, FromModifier::Optional(vec![Any]))
                .to_key(K::Tab, None)
                .build()],
        },
        Rule {
            description: S("VK1+O -> Ctrl+Shift+Tab / VK1+P -> Ctrl+Tab"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::O)
                    .to_key(K::Tab, Some(vec![Ctrl, Shift]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::P)
                    .to_key(K::Tab, Some(vec![Ctrl]))
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+Shift+Y -> Cmd+C and remove all newlines"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key_with_modifiers(K::Y, FromModifier::Mandatory(vec![Shift]))
                .to_key(K::C, Some(vec![Cmd]))
                .to_command(
                    "export LC_ALL=en_US.UTF-8; pbpaste | tr -d '\n' | sed 's/  */ /g' | pbcopy",
                )
                .build()],
        },
        Rule {
            description: S("VK1+Y -> Cmd+C"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key(K::Y)
                .to_key(K::C, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("VK1+T -> Cmd+X, VK1+X -> Cmd+Shift+V"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::T)
                    .to_key(K::X, Some(vec![Cmd]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::X)
                    .to_key(K::V, Some(vec![Cmd, Shift, Opt]))
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+C -> Backspace / VK1+E -> Delete"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::C)
                    .to_key(K::DeleteOrBackspace, None)
                    .build(),
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(K::E)
                    .to_key(K::DeleteForward, None)
                    .build(),
            ],
        },
        Rule {
            description: S("VK1+[ -> Cmd+Z"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key_with_modifiers(K::CloseBracket, FromModifier::Optional(vec![Any]))
                .to_key(K::Z, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("VK1+Colon -> Cmd+H"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key(K::Quote)
                .to_key(K::H, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("VK1+{N,M,Comma,Period} -> Mouse{Left,Down,Up,Right}"),
            manipulators: {
                let shift_mappings = vec![
                    (K::N, Some(-1536), None),
                    (K::M, None, Some(1536)),
                    (K::Comma, None, Some(-1536)),
                    (K::Period, Some(1536), None),
                ]
                .into_iter()
                .map(|(key_code, x, y)| {
                    Manipulator::builder()
                        .condition(Condition::with_virtual_key(VK::Vk1))
                        .from_key_with_modifiers(key_code, FromModifier::Mandatory(vec![Shift]))
                        .to_mouse(MouseKey {
                            x,
                            y,
                            vertical_wheel: None,
                        })
                        .build()
                });
                let normal_mappings = vec![
                    (K::N, Some(-3072), None),
                    (K::M, None, Some(3072)),
                    (K::Comma, None, Some(-3072)),
                    (K::Period, Some(3072), None),
                ]
                .into_iter()
                .map(|(key_code, x, y)| {
                    Manipulator::builder()
                        .condition(Condition::with_virtual_key(VK::Vk1))
                        .from_key(key_code)
                        .to_mouse(MouseKey {
                            x,
                            y,
                            vertical_wheel: None,
                        })
                        .build()
                });
                shift_mappings.chain(normal_mappings).collect()
            },
        },
        Rule {
            description: S("VK1+Slash -> LeftClick / VK1+Underscore -> RightClick"),
            manipulators: vec![
                (K::Slash, PointingButton::Button1),
                (K::International1, PointingButton::Button2),
            ]
            .into_iter()
            .map(|(from_key, pointing_button)| {
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key_with_modifiers(from_key, FromModifier::Optional(vec![Any]))
                    .to_click(pointing_button)
                    .build()
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
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key(key_code)
                    .to_mouse(MouseKey {
                        x: None,
                        y: None,
                        vertical_wheel: Some(vertical_wheel),
                    })
                    .build()
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
                Manipulator::builder()
                    .condition(Condition::with_virtual_key(VK::Vk1))
                    .from_key_with_modifiers(from, FromModifier::Optional(vec![Any]))
                    .to_key(to, None)
                    .build()
            })
            .collect_vec(),
        },
        Rule {
            description: S("VK1+B -> Ctrl+Opt+Cmd+Shift+M (Maximize window size with ShiftIt)"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key(K::B)
                .to_key(K::M, Some(vec![Ctrl, Opt, Cmd, Shift]))
                .build()],
        },
        Rule {
            description: S("VK1+Backslash -> Cmd+Opt+D (Hide the Dock)"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::with_virtual_key(VK::Vk1))
                .from_key(K::International3)
                .to_key(K::D, Some(vec![Cmd, Opt]))
                .build()],
        },
    ]
}
