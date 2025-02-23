use crate::karabiner_data::{KeyCode as K, ModifierKey::*, VirtualKey as VK, *};

pub fn manipulators() -> Vec<Manipulator> {
    vec![
        (K::Lang1, VK::Vk1, Some(K::JapaneseKana)),
        (K::International4, VK::Vk1, Some(K::JapaneseKana)),
        (K::Lang2, VK::Vk2, Some(K::JapaneseEisuu)),
        (K::International5, VK::Vk2, Some(K::JapaneseEisuu)),
        (K::RightGui, VK::Vk3, None),
        (K::International2, VK::Vk3, None),
        (K::Tab, VK::Vk4, Some(K::Tab)),
    ]
    .into_iter()
    .map(|(key_code, virtual_key, to_if_alone)| {
        let mut builder = Manipulator::builder()
            .from_key_with_modifiers(key_code, FromModifier::Optional(vec![Any]))
            .to_variable(SetVariable {
                name: virtual_key.clone(),
                value: 1,
            })
            .to_after_key_up(SetVariable {
                name: virtual_key,
                value: 0,
            });
        if let Some(to_if_alone) = to_if_alone {
            builder = builder.to_if_alone(to_if_alone);
        }
        builder.build()
    })
    .collect()
}
