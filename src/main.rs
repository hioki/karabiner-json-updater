mod config;
mod config_updater;
mod custom_rules_builder;

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

use crate::{config_updater::ConfigUpdater, custom_rules_builder::CustomRulesBuilder};

fn main() -> Result<()> {
    let config_dir = get_config_dir()?;
    let custom_rules_builder = CustomRulesBuilder::new();
    let custom_rules = custom_rules_builder.build();
    let config_updater = ConfigUpdater::new(config_dir, custom_rules);
    config_updater.update()
}

// https://karabiner-elements.pqrs.org/docs/json/location/
fn get_config_dir() -> Result<PathBuf> {
    let config_dir = Path::new(env!("HOME")).join(".config/karabiner");
    if !config_dir.is_dir() {
        return Err(anyhow!(format!(
            "{} must be created via Karabiner-Elements",
            config_dir.to_str().unwrap()
        )));
    }
    Ok(config_dir)
}
