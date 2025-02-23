use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};
use big_s::S;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("[Dynalist] VK1+U/I -> Ctrl+A/E"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk1())
                    .from_key(K::U)
                    .to_key(K::A, Some(vec![Ctrl]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk1())
                    .from_key(K::I)
                    .to_key(K::E, Some(vec![Ctrl]))
                    .build(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+E -> Toggle file pane"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Dynalist))
                .condition(Condition::with_vk4())
                .from_key(K::E)
                .to_key(K::F, Some(vec![Cmd, Shift]))
                .build()],
        },
        Rule {
            description: S("[Dynalist] VK4+F -> Open file finder"),
            manipulators: vec![Manipulator::builder()
                .condition(Condition::on_app(BundleIdentifier::Dynalist))
                .condition(Condition::with_vk4())
                .from_key(K::F)
                .to_key(K::O, Some(vec![Cmd]))
                .build()],
        },
        Rule {
            description: S("[Dynalist] VK2+9/0 -> Cmd+Shift+Hyphen/Cmd+Hyphen"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk2())
                    .from_key(K::Key9)
                    .to_key(K::Hyphen, Some(vec![Cmd, Shift]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk2())
                    .from_key(K::Key0)
                    .to_key(K::Hyphen, Some(vec![Cmd]))
                    .build(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+J/K -> Cmd+DownArrow/Cmd+UpArrow"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk4())
                    .from_key(K::J)
                    .to_key(K::DownArrow, Some(vec![Cmd]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk4())
                    .from_key(K::K)
                    .to_key(K::UpArrow, Some(vec![Cmd]))
                    .build(),
            ],
        },
        Rule {
            description: S("[Dynalist] VK4+H/L -> Shift+Tab/Tab"),
            manipulators: vec![
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk4())
                    .from_key(K::H)
                    .to_key(K::Tab, Some(vec![Shift]))
                    .build(),
                Manipulator::builder()
                    .condition(Condition::on_app(BundleIdentifier::Dynalist))
                    .condition(Condition::with_vk4())
                    .from_key(K::L)
                    .to_key(K::Tab, None)
                    .build(),
            ],
        },
    ]
}
