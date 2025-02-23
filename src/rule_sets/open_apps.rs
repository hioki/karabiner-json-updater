use crate::karabiner_data::{KeyCode as K, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
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
            Manipulator::builder()
                .condition(Condition::with_vk2())
                .from_key(key_code)
                .to_command(shell_command)
                .build()
        })
        .collect_vec(),
    }]
}
