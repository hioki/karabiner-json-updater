use anyhow::Result;
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::{copy, Read, Seek as _, SeekFrom, Write as _},
    path::PathBuf,
};

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

        // Update $HOME/.config/karabiner/karabiner.json
        let mut f = File::open(&self.karabiner_json_path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        let mut config: Value = serde_json::from_str(&s).unwrap();
        if let Some(profiles) = config["profiles"].as_array_mut() {
            if let Some(profile) = profiles.get_mut(0) {
                if let Some(complex_modifications) =
                    profile["complex_modifications"].as_object_mut()
                {
                    complex_modifications.insert("rules".to_string(), json!(&self.rules));
                }
            }
        }
        let new_json = serde_json::to_vec_pretty(&config).unwrap();
        let mut karabiner_json_file = File::create(&self.karabiner_json_path)?;
        let _written = karabiner_json_file.write(&new_json)?;

        Ok(())
    }
}
