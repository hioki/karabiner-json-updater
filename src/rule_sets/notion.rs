use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[Notion] VK2+9 -> Cmd+Shift+;"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk2())
                .from_key(K::Key9)
                .to_key(K::Semicolon, Some(vec![Cmd, Shift]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+E -> Cmd+Yen"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk4())
                .from_key(K::E)
                .to_key(K::International3, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+F -> Cmd+P"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk4())
                .from_key(K::F)
                .to_key(K::P, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+H -> Cmd+["),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk4())
                .from_key(K::H)
                .to_key(K::CloseBracket, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+L -> Cmd+]"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk4())
                .from_key(K::L)
                .to_key(K::NonUsPound, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+U -> Cmd+Shift+U"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk4())
                .from_key(K::U)
                .to_key(K::U, Some(vec![Cmd, Shift]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK2+9 -> Cmd+^"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk2())
                .from_key(K::Key9)
                .to_key(K::EqualSign, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK2+0 -> Cmd+Hyphen"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Notion))
                .condition(Condition::with_vk2())
                .from_key(K::Key0)
                .to_key(K::Hyphen, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Notion] VK4+N/P -> Ctrl+Shift+J/K"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Notion))
                    .condition(Condition::with_vk4())
                    .from_key(K::N)
                    .to_key(K::J, Some(vec![Ctrl, Shift]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Notion))
                    .condition(Condition::with_vk4())
                    .from_key(K::P)
                    .to_key(K::K, Some(vec![Ctrl, Shift]))
                    .build(),
            ],
        },
    ]
}
