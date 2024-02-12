//! # withings-rs
//! A library for interfacing with the Withings API.
//!
//! # Description
//!
//! This library will authenticate with the Withings API Ouath2 client credentials flow.
//! Store the access token in a file and refresh the token when it expires.
//! Withings does not follow the Oauth2 spec exactly, so the library will handle the differences.
//! Withings also does not return the same value type for userid, so we ignore that field.
//! In order to use this library you will need to register your application with Withings.
//! Then store the client_id and client_secret as an environment variables.
//! To store the config file outside the default of config.json, set the environment variable WITHINGS_CONFIG_FILE.
//!
//! # Examples
//!
//!  ```no_run
//! use withings_rs::{api::{auth, measure}, models::meas::CategoryType};
//! use std::env;
//! use simple_logger::SimpleLogger;
//! use std::path::Path;
//! use withings_rs::models::MeasureType;
//!
//! fn main () {
//!    println!("testing withings-rs\n");
//!
//!    // Initialize the logger to see the output
//!    SimpleLogger::new().init().unwrap();
//!   
//!    // Get the client id from the environment variable
//!    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
//!    
//!    // Get the config file if it exists or create a new one
//!    let config_file = auth::get_config_file();
//!    
//!    // Get the access token from the config file or get a new one
//!    let access_token = get_access_token(config_file);
//!    
//!    // Get the CategoryType and MeasureType
//!    let weight = MeasureType::Weight.to_string();
//!    
//!    // Get the measurements
//!    let measurements = measure::get_measurements(&access_token.unwrap().to_string(), &client_id, "1", &weight, None, None, None, Some("1706108118")).unwrap();
//!    println!("weight: {:?}", measurements.body.measuregrps[0].measures[0].value);
//! }
//!
//! // Get the access token from the config file or get a new one
//! fn get_access_token(config_file: String) -> Result<String, String>{
//!    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
//!    let client_secret = env::var("WITHINGS_CLIENT_SECRET").unwrap();
//!    
//!    // Check if the config file exists and get the access token or get a new one
//!    if Path::new(&config_file).exists() {
//!        let access_token = auth::refresh_token(client_id, client_secret);
//!        Ok(access_token)
//!    } else {
//!        let access_token = auth::get_access_code(client_id, client_secret);
//!        Ok(access_token)
//!    }
//! }
//! ```
//!
//! ## Extra
//!
//! There is an auth redirect server included in the redirect module. It will listen for the redirect on port 8888
//! from the Withings API and return the code and state. It does not have to be used, but it is included in the library.
//! A user could use their own server to listen for the redirect. It needs to return a HashMap with the code and state.
//!
//! ## Disclaimer
//!
//! This library is not affiliated with Withings. Use at your own risk.
//! This library currently only pulls in user measurements.

pub mod api;
pub mod models;
pub mod redirect;
