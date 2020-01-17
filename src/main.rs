mod config;
mod updater;

use crate::config::config::Config;
use crate::updater::updater::Updater;

fn main() -> Result<(), String> {
    let config = Config::my_config();
    let updater = Updater { config };

    updater.update()?;

    println!("done.");

    Ok(())
}
