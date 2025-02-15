use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    pub async fn load() -> Result<Config, String> {
        match File::options().read(true).open("config.toml") {
            Ok(mut file) => {
                let mut content = String::new();
                if let Err(error) = file.read_to_string(&mut content) {
                    return Err(format!("config error: {:?}", error.to_string()));
                }

                match toml::from_str::<Config>(&content) {
                    Ok(config) => Ok(config),
                    Err(error) => Err(format!("config error: {:?}", error.to_string())),
                }
            }
            Err(error) => Err(format!("config error: {:?}", error.to_string())),
        }
    }
}

#[tokio::test]
async fn test_config_load() {
    let config = Config::load().await;
    assert!(config.is_ok());
}
