use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            description: S("VK2+F -> Cmd+Tab / VK2+D -> Cmd+Shift+Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::F,
                    None,
                    K::Tab,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::D,
                    None,
                    K::Tab,
                    Some(vec![Cmd, Shift]),
                ),
            ],
        },
        Rule {
            description: S("VK2+S -> Ctrl+Tab / VK2+A -> Ctrl+Shift+Tab"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::S,
                    None,
                    K::Tab,
                    Some(vec![Ctrl]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::A,
                    None,
                    K::Tab,
                    Some(vec![Ctrl, Shift]),
                ),
            ],
        },
        Rule {
            description: S("VK2+9 -> Cmd+KeypadPlus / VK2+0 -> Cmd+Hyphen"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key9,
                    None,
                    K::KeypadPlus,
                    Some(vec![Cmd]),
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key0,
                    None,
                    K::Hyphen,
                    Some(vec![Cmd]),
                ),
            ],
        },
        Rule {
            description: S("VK2+1 -> VolumeDecrement / VK2+2 -> VolumeIncrement"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key1,
                    None,
                    K::VolumeDecrement,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key2,
                    None,
                    K::VolumeIncrement,
                    None,
                ),
            ],
        },
        Rule {
            description: S("VK2+3 -> BrightnessDecrement / VK2+4 -> BrightnessIncrement"),
            manipulators: vec![
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key3,
                    None,
                    K::DisplayBrightnessDecrement,
                    None,
                ),
                Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                    VK::Vk2,
                    K::Key4,
                    None,
                    K::DisplayBrightnessIncrement,
                    None,
                ),
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
                    Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                        VK::Vk2,
                        from,
                        Some(FromModifier::Mandatory(vec![Ctrl])),
                        to,
                        Some(vec![Cmd, Ctrl, Opt, Shift]),
                    )
                })
                .collect_vec(),
        },
    ]
}
