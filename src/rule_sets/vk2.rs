use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK2+F -> Cmd+Tab / VK2+D -> Cmd+Shift+Tab"),
            manipulators: vec![
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::F).to_key(K::Tab, Some(vec![Cmd])).build(),
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::D).to_key(K::Tab, Some(vec![Cmd,Shift])).build(),
            ],
        },
        Rule {
            description: S("VK2+S -> Ctrl+Tab / VK2+A -> Ctrl+Shift+Tab"),
            manipulators: vec![
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::S).to_key(K::Tab, Some(vec![Ctrl])).build(),
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::A).to_key(K::Tab, Some(vec![Ctrl,Shift])).build(),
            ],
        },
        Rule {
            description: S("VK2+9 -> Cmd+KeypadPlus / VK2+0 -> Cmd+Hyphen"),
            manipulators: vec![
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key9).to_key(K::KeypadPlus, Some(vec![Cmd])).build(),
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key0).to_key(K::Hyphen, Some(vec![Cmd])).build(),
            ],
        },
        Rule {
            description: S("VK2+1 -> VolumeDecrement / VK2+2 -> VolumeIncrement"),
            manipulators: vec![
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key1).to_key(K::VolumeDecrement, None).build(),
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key2).to_key(K::VolumeIncrement, None).build(),
            ],
        },
        Rule {
            description: S("VK2+3 -> BrightnessDecrement / VK2+4 -> BrightnessIncrement"),
            manipulators: vec![
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key3).to_key(K::DisplayBrightnessDecrement, None).build(),
                Manipulator::builder().condition(Condition::with_vk2()).from_key(K::Key4).to_key(K::DisplayBrightnessIncrement, None).build(),
            ],
        },
        Rule {
            description: S("VK2+Ctrl+{H,O,N,P,U,I,M,Comma,Period} -> Cmd+Ctrl+Opt+Shift+{Left,Right,Down,Up,1,2,3,4,N} (ShiftIt)"),
            manipulators: vec![
                (K::H, K::LeftArrow),
                (K::O, K::RightArrow),
                (K::N, K::DownArrow),
                (K::P, K::UpArrow),
                (K::U, K::Key1),
                (K::I, K::Key2),
                (K::M, K::Key3),
                (K::Comma, K::Key4),
                (K::J, K::P),
                (K::K, K::N),
            ]
                .into_iter()
                .map(|(from, to)| {
                    Manipulator::builder().condition(Condition::with_vk2()).from_key_with_modifiers(from, FromModifier::Mandatory(vec![Ctrl])).to_key(to, Some(vec![Cmd, Ctrl, Opt, Shift])).build()
                })
                .collect_vec(),
        },
    ]
}
