#[macro_use]
extern crate rouille;

use config::Config;

mod config;
mod crypto;
mod db;
mod server;
mod store;
mod util;

fn main() {
    let config = init();
    start(config);
}

fn init() -> Config {
    db::init();
    let v = util::get_os_env_value("FLASHPAPER_STATIC_KEY")
        .expect("You must set \"FLASHPAPER_STATIC_KEY\"");
    let static_key =
        util::hex_decode(&v).expect("Could not decode hex value for \"FLASHPAPER_STATIC_KEY\"");
    Config::new(static_key)
}

fn start(config: Config) {
    server::start(config);
}
