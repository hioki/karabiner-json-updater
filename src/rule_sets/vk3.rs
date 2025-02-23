use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};
use big_s::S;
use itertools::*;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        description: S("VK3+{A,S,D,F,G,H,J,K,L,Semicolon,Quote} -> {1,2,3,4,5,6,7,8,9,0,-}"),
        manipulators: vec![
            (K::A, K::Key1),
            (K::S, K::Key2),
            (K::D, K::Key3),
            (K::F, K::Key4),
            (K::G, K::Key5),
            (K::H, K::Key6),
            (K::J, K::Key7),
            (K::K, K::Key8),
            (K::L, K::Key9),
            (K::Semicolon, K::Key0),
            (K::Quote, K::Hyphen),
        ]
        .into_iter()
        .map(|(from, to)| {
            Manipulator::new_for_key_to_key_mapping_with_single_virtual_key(
                VK::Vk3,
                from,
                Some(FromModifier::Optional(vec![Any])),
                to,
                None,
            )
        })
        .collect_vec(),
    }]
}
