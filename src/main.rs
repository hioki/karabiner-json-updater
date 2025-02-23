pub mod karabiner_data;
pub mod rule_sets;

use std::io::{Seek as _, Write as _};

fn main() -> anyhow::Result<()> {
    let home_dir = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .expect("HOME environment variable must be set");
    let rules = vec![
        rule_sets::virtual_key::rules(),
        rule_sets::iterm2::rules(),
        rule_sets::vscode::rules(),
        rule_sets::dynalist::rules(),
        rule_sets::slack::rules(),
        rule_sets::google_chrome::rules(),
        rule_sets::notion::rules(),
        rule_sets::chatgpt::rules(),
        rule_sets::vk1::rules(),
        rule_sets::vk2::rules(),
        rule_sets::open_apps::rules(),
        rule_sets::vk3::rules(),
        rule_sets::semicolon::rules(),
        rule_sets::singlequote::rules(),
        rule_sets::capslock::rules(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<karabiner_data::Rule>>();
    let custom_filename = "custom.json";
    // https://karabiner-elements.pqrs.org/docs/json/location/
    let config_dir = home_dir.join(".config/karabiner");
    if !config_dir.is_dir() {
        panic!("{:?} must be created via Karabiner-Elements", config_dir);
    }
    let mut current_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(custom_filename)?;
    // https://karabiner-elements.pqrs.org/docs/json/root-data-structure/#custom-json-file-in-configkarabinerassetscomplex_modifications
    #[derive(Debug, serde::Serialize)]
    struct CustomRules<'a> {
        pub title: &'a str,
        pub rules: &'a Vec<karabiner_data::Rule>,
    }
    serde_json::to_writer_pretty(
        &current_file,
        &CustomRules {
            title: "Personal rules",
            rules: &rules,
        },
    )?;
    let mut config_file = std::fs::File::create(
        config_dir
            .join("assets/complex_modifications")
            .join(custom_filename),
    )?;
    current_file.seek(std::io::SeekFrom::Start(0))?;
    std::io::copy(&mut current_file, &mut config_file)?;
    let karabiner_json_path = config_dir.join("karabiner.json");
    let mut config: serde_json::Value =
        serde_json::from_reader(&std::fs::File::open(&karabiner_json_path)?)?;
    config["profiles"]
        .as_array_mut()
        .unwrap()
        .get_mut(0)
        .unwrap()["complex_modifications"]
        .as_object_mut()
        .unwrap()
        .insert("rules".to_string(), serde_json::json!(&rules));
    let buf = serde_json::to_vec_pretty(&config)?;
    let mut f = std::fs::File::create(karabiner_json_path)?;
    let _written = f.write(&buf)?;
    Ok(())
}
