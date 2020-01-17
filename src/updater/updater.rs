use crate::config::config::Config;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Updater {
    pub config: Config,
}

impl Updater {
    pub fn update(&self) -> Result<(), String> {
        if !is_jq_installed() {
            return Err("jq must be installed".to_owned());
        }

        let config_dir = concat!(env!("HOME"), "/.config/karabiner");

        if !Path::new(config_dir).is_dir() {
            return Err(format!(
                "{} must be created via Karabiner-Elements",
                config_dir
            ));
        }

        let personal_rules_file = File::create("./personal_rules.json").unwrap();

        serde_json::to_writer_pretty(personal_rules_file, &self.config).unwrap();

        let complex_modification_file = File::create(format!(
            "{}/assets/complex_modifications/personal_rules.json",
            config_dir,
        ))
        .unwrap();

        serde_json::to_writer_pretty(complex_modification_file, &self.config).unwrap();

        let karabiner_json_path = format!("{}/karabiner.json", config_dir);

        let rules_json = serde_json::to_string_pretty(&self.config.rules).unwrap();

        let query = format!(".profiles[0].complex_modifications.rules = {}", rules_json);

        let new_json = Command::new("jq")
            .arg(query)
            .arg(&karabiner_json_path)
            .output()
            .unwrap()
            .stdout;

        let mut karabiner_json_file = File::create(&karabiner_json_path).unwrap();

        let _size = karabiner_json_file.write(&new_json).unwrap();

        Ok(())
    }
}

fn is_jq_installed() -> bool {
    let command = "type";
    let arg = "jq";

    Command::new(command)
        .arg(arg)
        .stdout(Stdio::null())
        .status()
        .unwrap()
        .success()
}
