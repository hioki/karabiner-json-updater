use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK4 on VSCode"),
            manipulators: vec![
                K::A,             // execute command
                K::B,             // show bookmarks
                K::E,             // Toggle sidebar
                K::F,             // search file
                K::G,             // GitLens: Open File on Remote
                K::H,             // Go Back
                K::I,             // 実装へ移動
                K::L,             // Go Forward
                K::O,             // open recent
                K::K,             // find in path
                K::R,             // reload window
                K::S,             // go to symbol
                K::V,             // アクティブファイルの相対パスをコピー
                K::Y,             // Toggle File Blame
                K::Key9,          // 表示の拡大
                K::Key0,          // 表示の縮小
                K::ReturnOrEnter, // workbench.action.tasks.reRunTask
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
        Rule {
            description: S("[VSCode] VK4+P -> Cmd+Shift+M (問題を開く)"),
            manipulators: vec![ManipulatorInit {
                conditions: Some(vec![
                    Condition::on_app(BundleIdentifier::VSCode),
                    Condition::with_vk4(),
                ]),
                from: FromInit {
                    key_code: K::P,
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
    ]
}
