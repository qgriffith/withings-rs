use crate::models;
use log::info;
use std::env;

pub fn get_config_file() -> String {
    let config_file =
        env::var("WITHINGS_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
    info!("Using config file: {}", config_file);
    config_file
}
fn save_to_file<T: serde::Serialize>(
    file_path: &str,
    object: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(file_path)?;
    serde_json::to_writer_pretty(file, object)?;
    Ok(())
}

fn read_from_file<T: serde::de::DeserializeOwned>(
    file_path: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let object = serde_json::from_reader(file)?;
    Ok(object)
}

// Usage in `write_config`
pub fn write_config(access_token: &String, refresh_token: &String) {
    let config = models::Config {
        access_token: access_token.clone(),
        refresh_token: refresh_token.clone(),
    };
    if let Err(e) = save_to_file(&get_config_file(), &config) {
        panic!("Failed to write config file: {}", e);
    }
}

// Usage in `load_config`
pub fn load_config() -> models::Config {
    read_from_file(&get_config_file()).unwrap_or_else(|e| {
        panic!("Failed to load config file: {}", e);
    })
}
