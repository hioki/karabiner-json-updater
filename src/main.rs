mod bundle_identifier;
mod condition;
mod config;
mod from;
mod key_code;
mod manipulator;
mod modifier_key;
mod mouse_key;
mod rule;
mod set_variable;
mod to;
mod value;
mod virtual_key;

use serde_json;

use bundle_identifier::BundleIdentifier;
use condition::Condition;
use config::Config;
use from::{From, FromModifier};
use key_code::*;
use manipulator::{Manipulator, ManipulatorType, ToAfterKeyUp, ToIfAlone};
use modifier_key::ModifierKey;
use mouse_key::MouseKey;
use rule::Rule;
use set_variable::SetVariable;
use to::{PointingButton, To};
use value::Value;
use virtual_key::VirtualKey;

fn main() {
    let rules = vec![
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
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: Some(SetVariable {
                        name: virtual_key.clone(),
                        value: Value::On.value(),
                    }),
                    key_code: None,
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: Some(vec![ToAfterKeyUp {
                    set_variable: SetVariable {
                        name: virtual_key,
                        value: Value::Off.value(),
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
                r#type: ManipulatorType::default(),
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::ITerm2),
                    Condition::with_virtual_key(VirtualKey::Vk4),
                ]),
                from: From {
                    key_code: key_code.clone(),
                    modifiers: None,
                },
                to: vec![
                    tmux_prefix(),
                    To {
                        set_variable: None,
                        key_code: Some(key_code),
                        modifiers: Some(vec![ModifierKey::Control]),
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    },
                ],
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        vscode_vk4_rule(
            "[VSCODE][VK4] 1 -> workbench.action.openSettingsJson",
            KeyCode::Key1,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] 2 -> workbench.action.openGlobalKeybindingsFile",
            KeyCode::Key2,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] 3 -> workbench.action.openGlobalKeybindings",
            KeyCode::Key3,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] 4 -> workbench.view.extensions",
            KeyCode::Key4,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] a -> workbench.action.toggleActivityBarVisibility",
            KeyCode::A,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] h -> workbench.action.toggleSidebarVisibility",
            KeyCode::H,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] j -> workbench.action.togglePanel",
            KeyCode::J,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] e -> workbench.files.action.focusFilesExplorer",
            KeyCode::E,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] l -> workbench.action.focusFirstEditorGroup",
            KeyCode::L,
        ),
        vscode_vk4_rule("[VSCODE][VK4] s -> workbench.view.search", KeyCode::S),
        vscode_vk4_rule(
            "[VSCODE][VK4] p -> workbench.action.problems.focus",
            KeyCode::P,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] o -> workbench.action.output.toggleOutput",
            KeyCode::O,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] c -> workbench.debug.action.toggleRepl",
            KeyCode::C,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] m -> workbench.action.terminal.focus",
            KeyCode::M,
        ),
        vscode_vk4_rule("[VSCODE][VK4] k -> workbench.action.quickOpen", KeyCode::K),
        vscode_vk4_rule(
            "[VSCODE][VK4] r -> References: Find All References",
            KeyCode::R,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] x -> workbench.action.showCommands",
            KeyCode::X,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] i -> workbench.action.switchWindow",
            KeyCode::I,
        ),
        vscode_vk4_rule("[VSCODE][VK4] y -> copyFilePath", KeyCode::Y),
        vscode_vk4_rule(
            "[VSCODE][VK4] close_bracket -> workbench.action.moveEditorLeftInGroup",
            KeyCode::CloseBracket,
        ),
        vscode_vk4_rule(
            "[VSCODE][VK4] non_us_pound -> workbench.action.moveEditorRightInGroup",
            KeyCode::NonUsPound,
        ),
        Rule {
            description: "[Terminal] o/p -> control+t control+p / control+t control+n",
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
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::P),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
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
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::N),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] VK2 + a/s -> control+t control+p / control+t control+n",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk2),
                    ]),
                    from: From {
                        key_code: KeyCode::A,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::P),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk2),
                    ]),
                    from: From {
                        key_code: KeyCode::S,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::N),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] VK2 + h -> backspace",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                from: From {
                    key_code: KeyCode::H,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::DeleteOrBackspace),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[Terminal] z/y -> copy",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::Z,
                        modifiers: None,
                    },
                    to: vec![
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::CloseBracket),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![
                        Condition::on_app(BundleIdentifier::ITerm2),
                        Condition::with_virtual_key(VirtualKey::Vk1),
                    ]),
                    from: From {
                        key_code: KeyCode::Y,
                        modifiers: None,
                    },
                    to: vec![
                        To {
                            key_code: Some(KeyCode::ReturnOrEnter),
                            modifiers: None,
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                        tmux_prefix(),
                        To {
                            key_code: Some(KeyCode::M),
                            modifiers: Some(vec![ModifierKey::Control]),
                            set_variable: None,
                            mouse_key: None,
                            pointing_button: None,
                            shell_command: None,
                        },
                    ],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[Terminal] u/i -> shift+0 / shift+4",
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
                    to: vec![To {
                        key_code: Some(KeyCode::Key0),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
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
                    to: vec![To {
                        key_code: Some(KeyCode::Key4),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] h/j/k/l -> cursor move",
            manipulators: vec![
                (KeyCode::H, KeyCode::LeftArrow),
                (KeyCode::J, KeyCode::DownArrow),
                (KeyCode::K, KeyCode::UpArrow),
                (KeyCode::L, KeyCode::RightArrow),
            ]
            .into_iter()
            .map(|(from, to)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: from,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    key_code: Some(to),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] f -> escape",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::F,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    key_code: Some(KeyCode::Escape),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] s/d -> shift+control+j/shift+control+; (Google Japanese Input)",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::S,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::J),
                        modifiers: Some(vec![ModifierKey::Shift, ModifierKey::Control]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::D,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: Some(vec![ModifierKey::Shift, ModifierKey::Control]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] a/z -> f10/f7",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::A,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::F10),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Z,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::F7),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] u/i -> command+left/command+right",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::U,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::LeftArrow),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::I,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::RightArrow),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] g -> tab",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::G,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    key_code: Some(KeyCode::Tab),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] o/p -> control+shift+tab/control+tab",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::O,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::P,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] y/t/x -> command+c/command+x/command+shift+v",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Y,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::C),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::T,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::X),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::X,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::V),
                        modifiers: Some(vec![
                            ModifierKey::Command,
                            ModifierKey::Shift,
                            ModifierKey::Option,
                        ]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] c/e -> backspace/delete",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::C,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DeleteOrBackspace),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::E,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DeleteForward),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] [ -> command+z",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::CloseBracket,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    key_code: Some(KeyCode::Z),
                    modifiers: Some(vec![ModifierKey::Command]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] colon -> command+h",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::Quote,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::H),
                    modifiers: Some(vec![ModifierKey::Command]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] n/m/comma/dot -> mouse move",
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
                to: vec![To {
                    key_code: None,
                    modifiers: None,
                    set_variable: None,
                    mouse_key: Some(MouseKey {
                        x,
                        y,
                        vertical_wheel: None,
                    }),
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] / -> left click, _ -> right click",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Slash,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: Some(PointingButton::Button1),
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::International1,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: Some(PointingButton::Button2),
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] @/] -> scroll",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::OpenBracket,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(-64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::NonUsPound,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                    from: From {
                        key_code: KeyCode::Backslash,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: None,
                        modifiers: None,
                        set_variable: None,
                        mouse_key: Some(MouseKey {
                            x: None,
                            y: None,
                            vertical_wheel: Some(64),
                        }),
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK1] numbers -> function keys",
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
            .map(|(from, to)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: from,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(to),
                    modifiers: None,
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "[VK1] b -> window maximize (ShiftIt)",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::B,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::M),
                    modifiers: Some(vec![
                        ModifierKey::Control,
                        ModifierKey::Option,
                        ModifierKey::Command,
                    ]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK1] \\ -> command+option+d (Hide the Dock)",
            manipulators: vec![Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk1)]),
                from: From {
                    key_code: KeyCode::International3,
                    modifiers: None,
                },
                to: vec![To {
                    key_code: Some(KeyCode::D),
                    modifiers: Some(vec![ModifierKey::Command, ModifierKey::Option]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "[VK2] f/d -> command+tab/command+shift+tab",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::F,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::D,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Command, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] s/a -> control+tab/control+shift+tab",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::S,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::A,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Tab),
                        modifiers: Some(vec![ModifierKey::Control, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 9/0 -> command+shift+;/command+hyphen",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key9,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: Some(vec![ModifierKey::Command, ModifierKey::Shift]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key0,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::Hyphen),
                        modifiers: Some(vec![ModifierKey::Command]),
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 1/2 -> volume decrement/increment",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key1,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::VolumeDecrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key2,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::VolumeIncrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] 3/4 -> display brightness decrement/increment",
            manipulators: vec![
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key3,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DisplayBrightnessDecrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                    from: From {
                        key_code: KeyCode::Key4,
                        modifiers: None,
                    },
                    to: vec![To {
                        key_code: Some(KeyCode::DisplayBrightnessIncrement),
                        modifiers: None,
                        set_variable: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    r#type: ManipulatorType::default(),
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "[VK2] ShiftIt",
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
            .map(|(from, to)| Manipulator {
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
                from: From {
                    key_code: from,
                    modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                },
                to: vec![To {
                    key_code: Some(to),
                    modifiers: Some(vec![
                        ModifierKey::Command,
                        ModifierKey::Control,
                        ModifierKey::Option,
                    ]),
                    set_variable: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                r#type: ManipulatorType::default(),
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        open_app_rule(
            "[VK2] j -> Google Chrome.app",
            KeyCode::J,
            "open -a 'Google Chrome.app'",
        ),
        open_app_rule("[VK2] k -> iTerm.app", KeyCode::K, "open -a 'iTerm.app'"),
        open_app_rule(
            "[VK2] l -> Alfred 4.app",
            KeyCode::L,
            "open -a 'Alfred 4.app'",
        ),
        open_app_rule(
            "[VK2] i -> IntelliJ IDEA.app",
            KeyCode::I,
            "open -a 'IntelliJ IDEA.app'",
        ),
        open_app_rule(
            "[VK2] e -> snip search by Alfred 4",
            KeyCode::E,
            r#"osascript -e "tell application \"Alfred 4\" to search \"snip \"""#,
        ),
        open_app_rule(
            "[VK2] / -> Slack.app",
            KeyCode::Slash,
            "open -a 'Slack.app'",
        ),
        open_app_rule(
            "[VK2] @ -> Mail.app",
            KeyCode::OpenBracket,
            "open -a 'Mail.app'",
        ),
        open_app_rule(
            "[VK2] t -> Microsoft To Do.app",
            KeyCode::T,
            "open -a 'Microsoft To Do.app'",
        ),
        open_app_rule("[VK2] g -> Atom.app", KeyCode::G, "open -a 'Atom.app'"),
        open_app_rule(
            "[VK2] b -> Tweetbot.app",
            KeyCode::B,
            "open -a 'Tweetbot.app'",
        ),
        open_app_rule("[VK2] m -> Skim.app", KeyCode::M, "open -a 'Skim.app'"),
        open_app_rule("[VK2] r -> Notes.app", KeyCode::R, "open -a 'Notes.app'"),
        open_app_rule(
            "[VK2] v -> Visual Studio Code.app",
            KeyCode::V,
            "open -a 'Visual Studio Code.app'",
        ),
        open_app_rule(
            "[VK2] w -> 1Password.app",
            KeyCode::W,
            "open -a '1Password.app'",
        ),
        Rule {
            description: "[VK3] a..: -> 1..-",
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
            .map(|(from, to)| Manipulator {
                r#type: ManipulatorType::default(),
                conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk3)]),
                from: From {
                    key_code: from,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(to),
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            })
            .collect::<Vec<Manipulator>>(),
        },
        Rule {
            description: "; -> enter",
            manipulators: vec![
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Shift])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::Semicolon),
                        modifiers: Some(vec![ModifierKey::Shift]),
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
                Manipulator {
                    r#type: ManipulatorType::default(),
                    conditions: None,
                    from: From {
                        key_code: KeyCode::Semicolon,
                        modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                    },
                    to: vec![To {
                        set_variable: None,
                        key_code: Some(KeyCode::ReturnOrEnter),
                        modifiers: None,
                        mouse_key: None,
                        pointing_button: None,
                        shell_command: None,
                    }],
                    to_after_key_up: None,
                    to_if_alone: None,
                },
            ],
        },
        Rule {
            description: "control+: -> '",
            manipulators: vec![Manipulator {
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code: KeyCode::Quote,
                    modifiers: Some(FromModifier::Mandatory(vec![ModifierKey::Control])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(KeyCode::Key7),
                    modifiers: Some(vec![ModifierKey::Shift]),
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
        Rule {
            description: "caps_lock -> vk_none",
            manipulators: vec![Manipulator {
                r#type: ManipulatorType::default(),
                conditions: None,
                from: From {
                    key_code: KeyCode::CapsLock,
                    modifiers: Some(FromModifier::Optional(vec![ModifierKey::Any])),
                },
                to: vec![To {
                    set_variable: None,
                    key_code: Some(KeyCode::VkNone),
                    modifiers: None,
                    mouse_key: None,
                    pointing_button: None,
                    shell_command: None,
                }],
                to_after_key_up: None,
                to_if_alone: None,
            }],
        },
    ];

    let config = Config {
        title: "Personal rules",
        rules,
    };

    println!("{}", serde_json::to_string(&config).unwrap());
}

fn tmux_prefix() -> To {
    To {
        set_variable: None,
        key_code: Some(KeyCode::T),
        modifiers: Some(vec![ModifierKey::Control]),
        mouse_key: None,
        pointing_button: None,
        shell_command: None,
    }
}

fn vscode_vk4_rule(description: &'static str, key_code: KeyCode) -> Rule {
    Rule {
        description,
        manipulators: vec![Manipulator {
            r#type: ManipulatorType::default(),
            conditions: Some(vec![
                Condition::on_app(BundleIdentifier::VSCode),
                Condition::with_virtual_key(VirtualKey::Vk4),
            ]),
            from: From {
                key_code: key_code.clone(),
                modifiers: None,
            },
            to: vec![To {
                set_variable: None,
                key_code: Some(key_code),
                modifiers: Some(vec![
                    ModifierKey::Control,
                    ModifierKey::Shift,
                    ModifierKey::Option,
                    ModifierKey::Command,
                ]),
                mouse_key: None,
                pointing_button: None,
                shell_command: None,
            }],
            to_after_key_up: None,
            to_if_alone: None,
        }],
    }
}

fn open_app_rule(
    description: &'static str,
    key_code: KeyCode,
    shell_command: &'static str,
) -> Rule {
    Rule {
        description,
        manipulators: vec![Manipulator {
            conditions: Some(vec![Condition::with_virtual_key(VirtualKey::Vk2)]),
            from: From {
                key_code,
                modifiers: None,
            },
            to: vec![To {
                key_code: None,
                modifiers: None,
                set_variable: None,
                mouse_key: None,
                pointing_button: None,
                shell_command: Some(shell_command),
            }],
            r#type: ManipulatorType::default(),
            to_after_key_up: None,
            to_if_alone: None,
        }],
    }
}
