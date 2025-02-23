use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("[ChatGPT] VK4+E -> Cmd+Ctrl+S"),
        manipulators: vec![Manipulator::builder()
            .condition(Condition::on_app(BundleIdentifier::ChatGPT))
            .condition(Condition::with_vk4())
            .from_key(K::E)
            .to_key(K::S, Some(vec![Cmd, Ctrl]))
            .build()],
    }]
}
