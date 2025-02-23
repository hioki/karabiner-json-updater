use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[GoogleChrome] VK4+M -> Cmd+Shift+M (Switch profile)"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::GoogleChrome))
                .condition(Condition::with_vk4())
                .from_key(K::M)
                .to_key(K::M, Some(vec![Cmd, Shift]))
                .build()],
        },
        Rule {
            description: S(
                "[GoogleChrome] VK4+N -> Cmd+Shift+M,Down,Down,Down,Enter (Toggle profile)",
            ),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::GoogleChrome))
                .condition(Condition::with_vk4())
                .from_key(K::N)
                .to_key(K::M, Some(vec![Cmd, Shift]))
                .to_key(K::DownArrow, None)
                .to_key(K::DownArrow, None)
                .to_key(K::DownArrow, None)
                .to_key(K::ReturnOrEnter, None)
                .build()],
        },
    ]
}
