use std::collections::HashMap;
use random_string::generate;
use crate::{models, api};
use crate::redirect;
use log::{info, trace, warn};
use std::env;

// This module handles the auth with the Withings API. It uses the client_id and client_secret
// provided by Withings to auth with their API and get an access token. The access token is then
// used to make requests to the Withings API. The access and refresh tokens are stored in a config
// file for future use. The access token expires after 1 hour and the refresh token expires after
// 1 year. The refresh token is used to get a new access token when the current access token expires.


// Get the config file from the environment variable WITHINGS_CONFIG_FILE or set it to config.json
pub fn get_config_file() -> String {
    let config_file = env::var("WITHINGS_CONFIG_FILE").unwrap_or_else(|_| {
        "config.json".to_string()
    });
    info!("Using config file: {}", config_file);
    config_file
}

// This fucntion auths with the Withings API and returns an access token
// It setups a redirect server to get the auth code from the Withings API
// It takes two arguments: client_id and client_secret which are provided by Withings
// It returns an access token which is used to make requests to the Withings API
pub fn get_access_code(client_id: String, client_secret: String) -> Result<String, Box<dyn std::error::Error>> {

    
    // Setup URLS for Withings API Auth, Token, and Redirect
    let auth_url = "https://account.withings.com/oauth2_user/authorize2".to_string();
    let redirect_url = "http://localhost:8888".to_string();

    // Setup Withings API Scope
    let scope = "user.info,user.metrics,user.activity".to_string();

    // Generate a random string for CSRF protection
    let charset = "ABCDEfghiJKLnmoQRStuvWxyZ1234567890";
    let state =  generate(12, charset);
    
    // Setup Withings API Action and Authorization Code required by their API for auth
    let grant_type = "authorization_code".to_string();
    let action = "requesttoken".to_string();

    // Build the auth URL
    let auth_url = format!("{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}", auth_url, client_id, redirect_url, scope, state);
    
    // Print the auth URL and start the redirect server
    println!("Browse to: {}\n", auth_url);
    let get_code = redirect::server::run();

    // Get the auth code from the redirect server
    let auth_code = get_code["code"].to_string();
    info!("Got Auth Code: {}", auth_code);

    // Check the CSRF token
    if get_code["state"] != state {
        warn!("CSRF token mismatch!");
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "CSRF token mismatch!")));
    }
    
    // Build the params for the token request
    let mut params = HashMap::new();
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);
    params.insert("grant_type", &grant_type  );
    params.insert("redirect_uri", &redirect_url);
    params.insert("code", &auth_code);
    params.insert("action", &action);

    // Make the token request
    let token_url = api::wapi_url("v2/oauth2/".to_string());
    let client = reqwest::blocking::Client::new();
    let response = client.post(token_url)
    .form(&params)
    .send();
    
    trace!("Auth API parameters: {:?}", params);

    // Check for errors from the API
    if response.is_err() {
        warn!("Auth API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    info!("Response: {:?}", response);
   
    // Get the access token from the response
    let response_struct = response.unwrap().json::<models::Response>().unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });

    let access_token = response_struct.body.access_token;
    let refresh_token = response_struct.body.refresh_token;
    info!("Got Access Token: {}", access_token);
    
    write_config(&access_token, &refresh_token);
    Ok(access_token)
  
}

// Write the access token and refresh token to the config file
fn write_config(access_token: &String, refresh_token: &String) {
   let config = models::Config {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string()
    };
    
    let get_file = get_config_file();

    let config_file = std::fs::File::create(get_file).unwrap_or_else(|e| {
        panic!("Couldn't create file: {}", e);
    });
    serde_json::to_writer_pretty(config_file, &config).unwrap_or_else(|e| {
        panic!("Couldn't write to file: {}", e);
    });
    load_config();
}

// Load the config file from JSON and return a Config struct
fn load_config() -> models::Config {
    let get_file = get_config_file();

    let config_file = std::fs::File::open(get_file).unwrap_or_else(|e| {
        warn!("Couldn't open file: {}", e);
        panic!("Couldn't open file: {}", e);
    });

    let config = serde_json::from_reader(config_file).unwrap_or_else(|e| {
        warn!("Couldn't read file: {}", e);
        panic!("Couldn't read file: {}", e);
    });

    trace!("Loaded config: {:?}", config);
    config
}

// Refresh the access token using withings refresh token loaded from the config file
pub fn refresh_token(client_id: String, client_secret: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = load_config();
    let grant_type = "refresh_token".to_string();
    let refresh_token = config.refresh_token;
    let action = "requesttoken".to_string();

    let mut params = HashMap::new();
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);
    params.insert("grant_type", &grant_type );
    params.insert("refresh_token", &refresh_token);
    params.insert("action", &action);

    trace!("Refresh Token API parameters: {:?}", params);

    // Make the refresh token request
    let token_url = api::wapi_url("v2/oauth2/".to_string());
    let client = reqwest::blocking::Client::new();
    let response = client.post(token_url)
    .form(&params)
    .send();

    if response.is_err() {
        warn!("Refresh API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }
    info!("Refresh Response: {:?}", response);
   
    // Get the access token from the response
     let response_struct = response.unwrap().json::<models::Response>().unwrap_or_else(|e| {
         panic!("Error: {}", e);
     });
     let access_token = response_struct.body.access_token;
     let refresh_token = response_struct.body.refresh_token;
     info!("Got Access Token: {}", access_token);
    
     write_config(&access_token, &refresh_token);
     Ok(access_token)
}