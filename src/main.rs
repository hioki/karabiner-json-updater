mod config;

use crate::config::config::Config;

fn main() {
    println!("{}", Config::my_config().to_json());
}
