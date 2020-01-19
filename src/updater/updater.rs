use crate::config::config::Config;
use anyhow::{anyhow, Result};
use std::fs::{File, OpenOptions};
use std::io::{copy, Seek as _, SeekFrom, Write as _};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Updater {
    pub config: Config,
}

const PERSONAL_RULES_JSON_PATH: &str = "./personal_rules.json";

impl Updater {
    pub fn update(&self) -> Result<()> {
        if !is_jq_installed()? {
            return Err(anyhow!("jq must be installed"));
        }

        let config_karabiner_path = Path::new(env!("HOME")).join(".config/karabiner");

        if !config_karabiner_path.is_dir() {
            return Err(anyhow!(format!(
                "{} must be created via Karabiner-Elements",
                config_karabiner_path.to_str().unwrap()
            )));
        }

        let mut personal_rules_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .read(true)
            .open(PERSONAL_RULES_JSON_PATH)?;

        serde_json::to_writer_pretty(&personal_rules_file, &self.config)?;

        personal_rules_file.seek(SeekFrom::Start(0))?;

        let mut complex_modification_file = File::create(
            config_karabiner_path.join("assets/complex_modifications/personal_rules.json"),
        )?;

        let _written = copy(&mut personal_rules_file, &mut complex_modification_file)?;

        let karabiner_json_path = config_karabiner_path.join("karabiner.json");

        let rules_json = serde_json::to_string_pretty(&self.config.rules)?;

        let new_json = Command::new("jq")
            .arg(format!(
                ".profiles[0].complex_modifications.rules = {}",
                rules_json
            ))
            .arg(&karabiner_json_path)
            .output()?
            .stdout;

        let mut karabiner_json_file = File::create(&karabiner_json_path)?;

        let _size = karabiner_json_file.write(&new_json)?;

        Ok(())
    }
}

fn is_jq_installed() -> Result<bool> {
    let command = "type";
    let arg = "jq";

    let exit_status = Command::new(command)
        .arg(arg)
        .stdout(Stdio::null())
        .status()?;

    Ok(exit_status.success())
}
