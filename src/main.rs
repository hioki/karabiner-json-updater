mod config;
mod updater;

use crate::config::config::Config;
use crate::updater::Updater;
use anyhow::Result;

fn main() -> Result<()> {
    let config = Config::my_config();
    let updater = Updater { config };

    updater.update()?;

    println!("done.");

    Ok(())
}
