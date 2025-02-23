use crate::karabiner_data::{KeyCode as K, ModifierKey::*, *};

pub fn manipulators() -> Vec<Manipulator> {
    let vk4_conditions = vec![
        Condition::on_app(BundleIdentifier::Slack),
        Condition::with_vk4(),
    ];
    vec![
        (K::T, K::T, vec![Cmd, Shift]),           // Threads
        (K::U, K::A, vec![Cmd, Shift]),           // All Unreads
        (K::E, K::D, vec![Cmd, Shift]),           // Toggle Sidebar
        (K::K, K::G, vec![Cmd]),                  // Search
        (K::F, K::K, vec![Cmd]),                  // Jump
        (K::B, K::S, vec![Cmd, Shift]),           // Bookmarks
        (K::D, K::X, vec![Cmd, Shift]),           // Strike through
        (K::OpenBracket, K::C, vec![Cmd, Shift]), // Code
        (K::C, K::C, vec![Cmd, Opt, Shift]),      // Code Block
        (K::Q, K::Key9, vec![Cmd, Shift]),        // Quote
    ]
    .into_iter()
    .map(|(from, to, modifiers)| {
        Manipulator::builder()
            .conditions(vk4_conditions.clone())
            .from_key(from)
            .to_key(to, Some(modifiers))
            .build()
    })
    .collect()
}
