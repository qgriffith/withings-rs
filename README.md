# Withings-rs
Withings API Client in Rust

## About
First attempt at a rust project. This will Oauth to the Withings API and pull in your data.

## Use
You first need to create an Dev account on Withings to get a `client_id` and `client_secret` [Withings Dev Portal](https://developer.withings.com). Set the redirect URL to `http://localhost:8888` this client will bind to that on the machine it is ran on. To store the tokens in a config file that isn't the default of `config.json` set an env var of `WITHINGS_CONFIG_FILE` the directory path needs to exist for the code does not currnetly attempt to create the directory structure

### Example use

```rust
use withings_rs::{api::{auth, measure}, models::meas::CategoryType};
use std::env;
use simple_logger::SimpleLogger;
use std::path::Path;
use withings_rs::models::MeasureType;

fn main () {
    println!("testing withings-rs\n");

    // Initialize the logger to see the output
    SimpleLogger::new().init().unwrap();

    // Get the client id from the environment variable
    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();

    // Get the config file if it exists or create a new one
    let config_file = auth::get_config_file();

    // Get the access token from the config file or get a new one
    let access_token = get_access_token(config_file);

    // Get the CategoryType and MeasureType
    let category = CategoryType::Measures.to_string();
    let weight = MeasureType::Weight.to_string();
     // set up the measure api arguments 
    let params = measure::MeasurementParams{
        access_token: access_token.unwrap().to_string(),
        client_id,
        category,
        meastype: weight,
        start: None,
        end: None,
        offset: None,
        lastupdate: Some("1706108118".to_string())
    };
    // Get the measurements
    let measurements = measure::get_measurements(
        &params
    ).unwrap();
    println!("weight: {:?}", measurements.body.measuregrps[0].measures[0].value);
}

// Get the access token from the config file or get a new one
fn get_access_token(config_file: String) -> Result<String, String>{
    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
    let client_secret = env::var("WITHINGS_CLIENT_SECRET").unwrap();
    
    // Check if the config file exists and get the access token or get a new one
    if Path::new(&config_file).exists() {
        let access_token = auth::refresh_token(client_id, client_secret);
        Ok(access_token.unwrap().to_string())
    } else {
        let access_token = auth::get_access_code(client_id, client_secret);
        Ok(access_token.unwrap().to_string())
    }
}

```

## Disclaimer
This library is not affiliated with Withings. Use at your own risk. 
This is very much a work in progress. Right now all it does is auth and pull the measure API.