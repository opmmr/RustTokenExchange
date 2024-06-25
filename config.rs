use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub blockchain_network: String,
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn new(env_file: &str) -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok();

        let env_settings = match fs::metadata(env_file) {
            Ok(_) => config::File::with_name(env_file),
            Err(_) => config::File::from_str("", config::FileFormat::Env),
        };

        let mut settings = config::Config::default();
        settings
            .merge(env_settings)
            .merge(config::Environment::with_prefix("APP").separator("__"))?;

        settings.try_into()
    }
}

fn main() {
    let config = Config::new(".env").unwrap();
    println!("{:?}", config);
}