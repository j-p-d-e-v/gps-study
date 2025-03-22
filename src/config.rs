use serde::Deserialize;
use std::env;
use std::{fs::File, io::Read};
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl Config {
    pub async fn load(file_path: Option<String>) -> Result<Config, String> {
        let file_path: String = if let Ok(value) = env::var("APP_CONFIG_PATH") {
            value
        } else if let Some(value) = file_path {
            value
        } else {
            "config.toml".to_string()
        };
        match File::options().read(true).open(file_path) {
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
    let config = Config::load(None).await;
    assert!(config.is_ok());
}
