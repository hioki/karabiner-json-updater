mod config;
mod custom_rules;
mod updater;

use anyhow::{anyhow, Result};
use std::path::Path;

use crate::{custom_rules::build_custom_rules, updater::Updater};

fn main() -> Result<()> {
    let filename = "custom.json";

    // https://karabiner-elements.pqrs.org/docs/json/location/
    let config_dir = Path::new(env!("HOME")).join(".config/karabiner");
    if !config_dir.is_dir() {
        return Err(anyhow!(format!(
            "{} must be created via Karabiner-Elements",
            config_dir.to_str().unwrap()
        )));
    }

    let karabiner_json_path = config_dir.join("karabiner.json");

    let custom_path = config_dir
        .join("assets/complex_modifications")
        .join(filename);

    let rules = build_custom_rules();

    let updater = Updater::new(filename, karabiner_json_path, custom_path, rules);

    updater.update()
}
