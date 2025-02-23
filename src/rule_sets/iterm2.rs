use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("Substitute TMUX prefix with VK4 on iTerm2"),
            manipulators: vec![K::C, K::H, K::J, K::K, K::L, K::N, K::P, K::S, K::V]
                .into_iter()
                .map(|key_code| {
                    Manipulator::builder()
                        .condition(Condition::on_app(BundleIdentifier::ITerm2))
                        .condition(Condition::with_vk4())
                        .from_key(key_code.clone())
                        .to_key(KeyCode::T, Some(vec![ModifierKey::Ctrl]))
                        .to_key(key_code, Some(vec![Ctrl]))
                        .build()
                })
                .collect_vec(),
        },
        Rule {
            description: S("Avoid Cmd+W in iTerm"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .from_key_with_modifiers(K::W, FromModifier::Mandatory(vec![Cmd]))
                .to_key(K::VkNone, None)
                .build()],
        },
        Rule {
            description: S("[iTerm2] VK1+O -> Ctrl+T Ctrl+P / VK1+P -> Ctrl+T Ctrl+N"),
            manipulators: vec![(K::O, K::P), (K::P, K::N)]
                .into_iter()
                .map(|(from, to)| {
                    Manipulator::builder()
                        .condition(Condition::on_app(BundleIdentifier::ITerm2))
                        .condition(Condition::with_vk1())
                        .from_key(from)
                        .to_key(KeyCode::T, Some(vec![Ctrl]))
                        .to_key(to, Some(vec![Ctrl]))
                        .build()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK2+A -> Ctrl+T Ctrl+P / VK2+S -> Ctrl+T Ctrl+N"),
            manipulators: vec![(K::A, K::P), (K::S, K::N)]
                .into_iter()
                .map(|(from, to)| {
                    Manipulator::builder()
                        .condition(Condition::on_app(BundleIdentifier::ITerm2))
                        .condition(Condition::with_vk2())
                        .from_key(from)
                        .to_key(KeyCode::T, Some(vec![Ctrl]))
                        .to_key(to, Some(vec![Ctrl]))
                        .build()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK1+W -> <ESC>:w<CR>"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .condition(Condition::with_vk1())
                .from_key(K::W)
                .to_key(K::Escape, None)
                .to_key(K::Quote, None)
                .to_key(K::W, None)
                .to_key(K::ReturnOrEnter, None)
                .build()],
        },
        Rule {
            description: S("[iTerm2] VK1+Q -> <ESC>:q<CR>"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .condition(Condition::with_vk1())
                .from_key(K::Q)
                .to_key(K::Escape, None)
                .to_key(K::Quote, None)
                .to_key(K::Q, None)
                .to_key(K::ReturnOrEnter, None)
                .build()],
        },
        Rule {
            description: S("[iTerm2] VK1+Z -> Enter tmux copy-mode"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .condition(Condition::with_vk1())
                .from_key(K::Z)
                .to_key(KeyCode::T, Some(vec![Ctrl]))
                .to_key(KeyCode::CloseBracket, Some(vec![Ctrl]))
                .build()],
        },
        Rule {
            description: S("[iTerm2] VK1+U -> Shift+0 / VK1+I -> shift+4"),
            manipulators: vec![(K::U, K::Key0), (K::I, K::Key4)]
                .into_iter()
                .map(|(from, to)| {
                    Manipulator::builder()
                        .condition(Condition::on_app(BundleIdentifier::ITerm2))
                        .condition(Condition::with_vk1())
                        .from_key(from)
                        .to_key(to, Some(vec![Shift]))
                        .build()
                })
                .collect_vec(),
        },
        Rule {
            description: S("[iTerm2] VK1+Semicolon -> Ctrl+F"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::ITerm2))
                .condition(Condition::with_vk1())
                .from_key(K::Semicolon)
                .to_key(K::F, Some(vec![Ctrl]))
                .build()],
        },
    ]
}
