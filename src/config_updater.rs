use anyhow::Result;
use big_s::S;
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::{copy, Seek as _, SeekFrom, Write as _},
    path::{Path, PathBuf},
};

use crate::config::rule::Rule;

static TITLE: &str = "Personal rules";
static FILENAME: &str = "custom.json";

pub struct ConfigUpdater {
    config_dir: PathBuf,
    rules: Vec<Rule>,
}

impl ConfigUpdater {
    pub fn new(config_dir: PathBuf, rules: Vec<Rule>) -> Self {
        Self { config_dir, rules }
    }

    pub fn update(&self) -> Result<()> {
        self.update_custom_rules_files()?;
        let karabiner_json_path = self.get_karabiner_json_path();
        let config: Value = self.get_updated_config(&karabiner_json_path)?;
        self.update_karabiner_json_file(&karabiner_json_path, &config)?;
        Ok(())
    }

    fn update_custom_rules_files(&self) -> Result<()> {
        let mut file_in_current = self.get_custom_rules_file_in_current_dir()?;
        let mut file_in_config = self.get_custom_rules_file_in_config_dir()?;
        let custom_rules = CustomRules {
            title: TITLE,
            rules: &self.rules,
        };
        serde_json::to_writer_pretty(&file_in_current, &custom_rules)?;
        file_in_current.seek(SeekFrom::Start(0))?;
        copy(&mut file_in_current, &mut file_in_config)?;
        Ok(())
    }

    fn get_updated_config(&self, karabiner_json_path: &Path) -> Result<Value> {
        let mut config: Value = serde_json::from_reader(&File::open(karabiner_json_path)?)?;
        config["profiles"]
            .as_array_mut()
            .unwrap()
            .get_mut(0)
            .unwrap()["complex_modifications"]
            .as_object_mut()
            .unwrap()
            .insert(S("rules"), json!(&self.rules));
        Ok(config)
    }

    fn update_karabiner_json_file(&self, karabiner_json_path: &Path, config: &Value) -> Result<()> {
        let buf = serde_json::to_vec_pretty(config)?;
        let mut f = File::create(karabiner_json_path)?;
        let _written = f.write(&buf)?;
        Ok(())
    }

    // $HOME/.config/karabiner/karabiner.json
    fn get_karabiner_json_path(&self) -> PathBuf {
        self.config_dir.join("karabiner.json")
    }

    // ./custom.json
    fn get_custom_rules_file_in_current_dir(&self) -> Result<File> {
        Ok(OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .read(true)
            .open(FILENAME)?)
    }

    // $HOME/.config/karabiner/assets/complex_modifications/custom.json
    fn get_custom_rules_file_in_config_dir(&self) -> Result<File> {
        Ok(File::create(
            self.config_dir
                .join("assets/complex_modifications")
                .join(FILENAME),
        )?)
    }
}

// https://karabiner-elements.pqrs.org/docs/json/root-data-structure/#custom-json-file-in-configkarabinerassetscomplex_modifications
#[derive(Debug, Serialize)]
struct CustomRules<'a> {
    pub title: &'a str,
    pub rules: &'a Vec<Rule>,
}
