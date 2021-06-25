pub struct Config {
    pub static_key: Vec<u8>,
}

impl Config {
    pub fn new(static_key: Vec<u8>) -> Config {
        Config { static_key }
    }
}
