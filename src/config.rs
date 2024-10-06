use serde::Deserialize;
use std::fs::read_to_string;

/// A struct describing the configuration schema
/// - user: A string describing the username for the relay server
/// - pwd: A string describing the password for the relay server
/// (this will usually be an app password similar to an API key)
/// - forward_address: A string describing the email address to forward messages to
/// - server: A string describing the server URL matching the username and pwd
/// - listen_address: A string describing the address and port the server will listen on
/// 
/// # Example Config.toml file that matches this schema
/// ```toml
/// user = "bob@mail.com"
/// pwd = "04 08 0F 10 17 2A"
/// forward_address = "alice@mail.com"
/// server = "smtp.mail.com" 
/// listen_address = "0.0.0.0:8080"
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub user: String,
    pub pwd: String,
    pub forward_address: String,
    pub server: String,
    pub listen_address: String,
}

impl Config {
    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = read_to_string(filename)
            .map_err(|err| format!("Unable to read config file: {}", err))?;
        let config: Config = toml::from_str(&config_str)
            .map_err(|err| format!("Unable to parse config file: {}", err))?;
        Ok(config)
    }
}