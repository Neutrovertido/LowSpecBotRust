use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub auto_reply_enabled: bool,
}

pub fn load_config() -> io::Result<Config> {
    let contents = fs::read_to_string(CONFIG_PATH)?;
    toml::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn save_config(config: &Config) -> io::Result<()> {
    let toml_string = toml::to_string(config)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(CONFIG_PATH, toml_string)
}
