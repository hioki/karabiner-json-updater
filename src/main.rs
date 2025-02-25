pub mod karabiner_data;
pub mod rule_sets;

const CUSTOM_JSON_FILENAME: &str = "custom.json";

fn main() -> anyhow::Result<()> {
    use std::io::{Seek as _, Write as _};

    // https://karabiner-elements.pqrs.org/docs/json/location/
    let config_dir = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .expect("HOME environment variable must be set")
        .join(".config/karabiner");
    if !config_dir.is_dir() {
        panic!("{:?} must be created via Karabiner-Elements", config_dir);
    }

    let rules = vec![karabiner_data::Rule {
        description: "Personal rules".to_string(),
        manipulators: vec![
            rule_sets::virtual_key::manipulators(),
            rule_sets::iterm2::manipulators(),
            rule_sets::vscode::manipulators(),
            rule_sets::dynalist::manipulators(),
            rule_sets::slack::manipulators(),
            rule_sets::google_chrome::manipulators(),
            rule_sets::notion::manipulators(),
            rule_sets::chatgpt::manipulators(),
            rule_sets::vk1::manipulators(),
            rule_sets::vk2::manipulators(),
            rule_sets::open_apps::manipulators(),
            rule_sets::vk3::manipulators(),
            rule_sets::semicolon::manipulators(),
            rule_sets::singlequote::manipulators(),
            rule_sets::capslock::manipulators(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<karabiner_data::Manipulator>>(),
    }];
    let complex_modifications = karabiner_data::ComplexModifications {
        title: "Personal rules",
        rules: &rules,
    };

    // 1. write custom.json
    let mut custom_json_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(CUSTOM_JSON_FILENAME)?;
    serde_json::to_writer_pretty(&custom_json_file, &complex_modifications)?;

    // 2. copy custom.json to karabiner assets (~/.config/karabiner/assets/complex_modifications/custom.json)
    let mut karabiner_assets_file = std::fs::File::create(
        config_dir
            .join("assets/complex_modifications")
            .join(CUSTOM_JSON_FILENAME),
    )?;
    custom_json_file.seek(std::io::SeekFrom::Start(0))?;
    std::io::copy(&mut custom_json_file, &mut karabiner_assets_file)?;

    // 3. update karabiner.json (~/.config/karabiner/karabiner.json)
    let karabiner_json_path = config_dir.join("karabiner.json");
    let mut karabiner_json: serde_json::Value =
        serde_json::from_reader(&std::fs::File::open(&karabiner_json_path)?)?;
    karabiner_json["profiles"]
        .as_array_mut()
        .unwrap()
        .get_mut(0)
        .unwrap()["complex_modifications"]
        .as_object_mut()
        .unwrap()
        .insert("rules".to_string(), serde_json::json!(&rules));
    let karabiner_json_data = serde_json::to_vec_pretty(&karabiner_json)?;
    let mut f = std::fs::File::create(karabiner_json_path)?;
    let _written = f.write(&karabiner_json_data)?;
    Ok(())
}
