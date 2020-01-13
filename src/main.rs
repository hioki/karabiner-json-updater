mod config;
mod updater;

use crate::config::config::Config;
use crate::updater::updater::Updater;
use std::process;

fn main() {
    let config = Config::my_config();
    let updater = Updater { config };

    process::exit(match updater.update() {
        Ok(_) => {
            println!("done.");
            0
        }
        Err(message) => {
            eprintln!("{}", message);
            1
        }
    })
}
