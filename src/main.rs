mod config;
mod config_updater;
mod custom_rules_builder;

use crate::{config_updater::ConfigUpdater, custom_rules_builder::CustomRulesBuilder};

fn main() -> anyhow::Result<()> {
    // https://karabiner-elements.pqrs.org/docs/json/location/
    let config_dir = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .expect("HOME environment variable must be set")
        .join(".config/karabiner");
    if !config_dir.is_dir() {
        anyhow::bail!("{:?} must be created via Karabiner-Elements", config_dir);
    }
    let custom_rules_builder = CustomRulesBuilder::new();
    let custom_rules = custom_rules_builder.build();
    let config_updater = ConfigUpdater::new(config_dir, custom_rules);
    config_updater.update()
}
