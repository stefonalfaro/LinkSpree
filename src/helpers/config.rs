use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub auth_token: String
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Read the file to a String
    let config_str = fs::read_to_string("config/config.json")?;

    // Deserialize the JSON string into the Config struct
    let config: Config = serde_json::from_str(&config_str)?;

    Ok(config)
}