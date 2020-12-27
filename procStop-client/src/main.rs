use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use rusqlite;

use procStop_client::*;

fn main() {
    let conf = conf::get_config("config.toml").unwrap();
    let components = init_components(conf).unwrap();
    loop {}
}
