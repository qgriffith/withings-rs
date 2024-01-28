# Withings-rs
Withings API Client in Rust

## About
First attempt at a rust project. This will Oauth to the Withings API and pull in your data.

## Use
You first need to create an Dev account on Withings to get a `client_id` and `client_secret` [Withings Dev Portal](https://developer.withings.com). Set the redirect URL to `http://localhost:8888` this client will bind to that on the machine it is ran on.

### Example use

```rust
use withings_rs::api::{auth, measure};
use std::env;
use simple_logger::SimpleLogger;
use std::path::Path;
use withings_rs::models::MeasureType;
fn main () {
    println!("testing withings-rs\n");
    SimpleLogger::new().init().unwrap();


    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
    let config_file = "config.json".to_string();
    let access_token = get_access_token(config_file);
    let weight = MeasureType::Weight.to_string();
    let measurements = measure::get_measurements(&access_token.unwrap().to_string(), &client_id, "1", &weight, None, None, None, Some("1706108118")).unwrap();
    println!("weight: {:?}", measurements.body.measuregrps[0].measures[0].value);
}

fn get_access_token(config_file: String) -> Result<String, String>{
    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
    let client_secret = env::var("WITHINGS_CLIENT_SECRET").unwrap();
    
    if Path::new(&config_file).exists() {
        let access_token = auth::refresh_token(client_id, client_secret);
        Ok(access_token)
    } else {
        let access_token = auth::get_access_code(client_id, client_secret);
        Ok(access_token)
    }
}
```

## Disclaimer
This is very much a work in progress. Right now all it does is auth to the API.