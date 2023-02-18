use anyhow::Result;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{copy, Seek as _, SeekFrom, Write as _};
use std::path::PathBuf;
use std::process::Command;

use crate::config::rule::Rule;

pub struct Updater<'a> {
    title: &'a str,
    filename: &'a str,
    karabiner_json_path: PathBuf,
    custom_path: PathBuf,
    rules: Vec<Rule>,
}

impl<'a> Updater<'a> {
    pub fn new(
        title: &'a str,
        filename: &'a str,
        karabiner_json_path: PathBuf,
        custom_path: PathBuf,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            title,
            filename,
            karabiner_json_path,
            custom_path,
            rules,
        }
    }

    pub fn update(&self) -> Result<()> {
        // https://karabiner-elements.pqrs.org/docs/json/root-data-structure/#custom-json-file-in-configkarabinerassetscomplex_modifications
        #[derive(Debug, Serialize)]
        struct Config<'a> {
            pub title: &'a str,
            pub rules: &'a Vec<Rule>,
        }

        // Update ./custom.json
        let mut custom_file_in_current_dir = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .read(true)
            .open(self.filename)?;
        serde_json::to_writer_pretty(
            &custom_file_in_current_dir,
            &Config {
                title: self.title,
                rules: &self.rules,
            },
        )?;

        // Update $HOME/.config/karabiner/assets/complex_modifications/custom.json
        custom_file_in_current_dir.seek(SeekFrom::Start(0))?;
        let mut custom_file_in_config = File::create(&self.custom_path)?;
        copy(&mut custom_file_in_current_dir, &mut custom_file_in_config)?;

        // Update $HOME/.config/karabiner.json
        // TODO: remove jq dependency
        let new_json = Command::new("jq")
            .arg(format!(
                ".profiles[0].complex_modifications.rules = {}",
                serde_json::to_string_pretty(&self.rules)?
            ))
            .arg(&self.karabiner_json_path)
            .output()?
            .stdout;
        let mut karabiner_json_file = File::create(&self.karabiner_json_path)?;
        let _written = karabiner_json_file.write(&new_json)?;

        Ok(())
    }
}
