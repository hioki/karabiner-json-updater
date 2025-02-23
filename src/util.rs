use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .expect("HOME environment variable must be set")
}
