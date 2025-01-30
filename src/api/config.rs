//! # Configuration Helpers
//! This module provides utilities for managing configuration files. It includes functions for
//! reading and writing configuration data, and for handling file paths tied to environment variables.

use crate::models;
use log::info;
use std::env;

/// Retrieves the file path for the Withings configuration file.
///
/// This function checks for the `WITHINGS_CONFIG_FILE` environment variable. If the environment
/// variable is not set, it defaults to `config.json`.
///
/// # Returns
/// A `String` containing the path to the configuration file.
///
pub fn get_config_file() -> String {
    let config_file =
        env::var("WITHINGS_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
    info!("Using config file: {}", config_file);
    config_file
}

/// Serializes an object and writes it to a file.
///
/// This function takes any object that implements the `serde::Serialize` trait,
/// serializes the object to JSON, and writes it to the provided file path.
///
/// # Arguments
/// - `file_path`: The path where the serialized object will be written.
/// - `object`: The object to be serialized and saved.
///
/// # Returns
/// - `Ok(())` on success.
/// - An error wrapped in `Result` if file creation or serialization fails.
///
fn save_to_file<T: serde::Serialize>(
    file_path: &str,
    object: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(file_path)?;
    serde_json::to_writer_pretty(file, object)?;
    Ok(())
}

/// Reads a JSON file and deserializes it into an object.
///
/// This function takes the path to a file and reads its content, parsing the JSON
/// into an object that implements the `serde::de::DeserializeOwned` trait.
///
/// # Arguments
/// - `file_path`: Path to the JSON file to read and deserialize.
///
/// # Returns
/// - On success, returns the deserialized object of type `T`.
/// - An error wrapped in `Result` if file reading or JSON deserialization fails.
///
fn read_from_file<T: serde::de::DeserializeOwned>(
    file_path: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let object = serde_json::from_reader(file)?;
    Ok(object)
}

/// Writes a new configuration to the configuration file.
///
/// This function creates a `Config` object, based on the provided `access_token`
/// and `refresh_token`, and saves it to the configuration file specified by
/// the `get_config_file()` function.
///
/// # Arguments
/// - `access_token`: The user's access token.
/// - `refresh_token`: The user's refresh token.
///
/// # Returns
/// - `Ok(())` on success.
/// - An error wrapped in `Result` if the file operation fails.
///
// Usage in `write_config`
pub fn write_config(
    access_token: &String,
    refresh_token: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = models::Config {
        access_token: access_token.clone(),
        refresh_token: refresh_token.clone(),
    };
    save_to_file(&get_config_file(), &config)
}

/// Loads the configuration from the configuration file.
///
/// This function reads the configuration file specified by `get_config_file()`
/// and parses its content into a `Config` object.
///
/// # Returns
/// - On success, returns a `Config` struct populated with the file's data.
/// - An error wrapped in `Result` if file reading or JSON deserialization fails.
///
pub fn load_config() -> Result<models::Config, Box<dyn std::error::Error>> {
    read_from_file(&get_config_file())
}
