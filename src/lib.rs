//! A library for interfacing with the Withings API.
//! 
//! # Description
//! 
//! This library will authenticate with the Withings API Ouath2 client credentials flow.
//! Withings does not follow the Oauth2 spec exactly, so the library will handle the differences.
//! Withings also does not return the same value type for userid so we ignore that field.
//! In order to use this library you will need to register your application with Withings.
//! Then store the client_id and client_secret as an environment variables.
//! 
//! # Examples
//! 
//!  ```no_run
//! use withings_rs::api::{auth, measure};
//! use std::env;
//! !use simple_logger::SimpleLogger;
//! !use std::path::Path;
//! use withings_rs::models::MeasureType;
//! fn main () {
//!    println!("testing withings-rs\n");
//!    SimpleLogger::new().init().unwrap();
//!    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
//!    let config_file = "config.json".to_string();
//!    let access_token = get_access_token(config_file);
//!    let weight = MeasureType::Weight.to_string();
//!    let measurements = measure::get_measurements(&access_token.unwrap().to_string(), &client_id, "1", &weight, None, None, None, Some("1706108118")).unwrap();
//!    println!("weight: {:?}", measurements.body.measuregrps[0].measures[0].value);
//! }
//!
//! fn get_access_token(config_file: String) -> Result<String, String>{
//!    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
//!    let client_secret = env::var("WITHINGS_CLIENT_SECRET").unwrap();
//!    
//!    if Path::new(&config_file).exists() {
//!        let access_token = auth::refresh_token(client_id, client_secret);
//!        Ok(access_token)
//!    } else {
//!        let access_token = auth::get_access_code(client_id, client_secret);
//!        Ok(access_token)
//!    }
//! }
//! ```

pub mod api;
pub mod models;
pub mod redirect;
