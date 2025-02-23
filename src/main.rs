pub mod karabiner_data;
pub mod rule_sets;

const CUSTOM_JSON_FILENAME: &str = "custom.json";

fn gather_all_rules() -> Vec<karabiner_data::Rule> {
    vec![
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
    .collect::<Vec<karabiner_data::Rule>>()
}

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

    let rules = gather_all_rules();

    // 1. write custom.json
    let mut custom_json_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open(CUSTOM_JSON_FILENAME)?;
    // https://karabiner-elements.pqrs.org/docs/json/root-data-structure/#custom-json-file-in-configkarabinerassetscomplex_modifications
    #[derive(Debug, serde::Serialize)]
    struct CustomRules<'a> {
        pub title: &'a str,
        pub rules: &'a Vec<karabiner_data::Rule>,
    }
    serde_json::to_writer_pretty(
        &custom_json_file,
        &CustomRules {
            title: "Personal rules",
            rules: &rules,
        },
    )?;
    return Ok(()); // TODO: remove

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
