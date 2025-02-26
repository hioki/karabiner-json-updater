use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        vec![
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
            K::J,             // Toggle Copilot Edits
        ]
        .into_iter()
        .map(|key_code| {
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(key_code.clone())
                .to_key(key_code, Some(vec![Ctrl, Shift, Opt, Cmd]))
                .build()
        })
        .collect(),
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::J)
                .to_key(K::S, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk1())
                .from_key(K::W)
                .to_key(K::S, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::M)
                .to_key(K::K, Some(vec![Opt, Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::U)
                .to_key(K::F12, Some(vec![Shift]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::N)
                .to_key(K::F8, Some(vec![Opt]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::Period)
                .to_key(K::Period, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::T)
                .to_key(K::T, Some(vec![Cmd]))
                .build(),
        ],
        vec![
            Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::VSCode))
                .condition(Condition::with_vk4())
                .from_key(K::P)
                .to_key(K::M, Some(vec![Cmd, Shift]))
                .build(),
        ],
    ]
    .into_iter()
    .flatten()
    .collect()
}
