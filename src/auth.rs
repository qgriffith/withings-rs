use std::collections::HashMap;
use random_string::generate;
use crate::models;
use crate::redirect;
use log::{info, warn};

// This fucntion auths with the Withings API and returns an access token
// It takes two arguments: client_id and client_secret which are provided by Withings
// This is a very basic implementation of the Withings API auth flow
// TODO: Break this function up into smaller functions
// TODO: Add error handling and logging

static CONFIG_F: &str = "config.json"; //#TODO: store this in a better place

pub fn get_access_code(client_id: String, client_secret: String) -> String {

    
    // Setup URLS for Withings API Auth, Token, and Redirect
    let auth_url = "https://account.withings.com/oauth2_user/authorize2".to_string();
    let token_url = "https://wbsapi.withings.net/v2/oauth2".to_string();
    let redirect_url = "http://localhost:8888".to_string();

    // Setup Withings API Scope
    let scope = "user.info,user.metrics,user.activity".to_string();

    // Generate a random string for CSRF protection
    let charset = "ABCDEfghiJKLnmoQRStuvWxyZ1234567890";
    let state =  generate(12, charset);
    
    // Setup Withings API Action and Authorization Code required by their API for auth
    let authorization_code = "authorization_code".to_string();
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
        warn!("CSRF token mismatch :(");
        panic!("CSRF token mismatch :(");
    }
    
    // Build the params for the token request
    let mut params = HashMap::new();
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);
    params.insert("grant_type", &authorization_code );
    params.insert("redirect_uri", &redirect_url);
    params.insert("code", &auth_code);
    params.insert("action", &action);

    // Make the token request
    let client = reqwest::blocking::Client::new();
    let response = client.post(token_url)
    .form(&params)
    .send();
   
    // Get the access token from the response
    let response_struct = response.unwrap().json::<models::Response>().unwrap();
    let access_token = response_struct.body.access_token.clone(); //TODO: fix using clone
    info!("Got Access Token: {}", access_token);
    
    write_config(response_struct);
    access_token
  
}

fn write_config(response: models::Response) {
   let config = models::Config {
        access_token: response.body.access_token,
        refresh_token: response.body.refresh_token,
    };
    
    let config_file = std::fs::File::create(&CONFIG_F).unwrap();
    serde_json::to_writer_pretty(config_file, &config).unwrap();
    load_config();

}

fn load_config() -> models::Config {
    let config_file = std::fs::File::open(&CONFIG_F).unwrap();
    let config = serde_json::from_reader(config_file).unwrap();
    println!("{:?}\n", config);
    config
}

pub fn refresh_token(client_id: String, client_secret: String) -> String {
    let config = load_config();
    let token_url = "https://wbsapi.withings.net/v2/oauth2".to_string();
    let grant_type = "refresh_token".to_string();
    let refresh_token = config.refresh_token;
    let action = "requesttoken".to_string();

    let mut params = HashMap::new();
    params.insert("client_id", &client_id);
    params.insert("client_secret", &client_secret);
    params.insert("grant_type", &grant_type );
    params.insert("refresh_token", &refresh_token);
    params.insert("action", &action);

    // Make the refresh token request
    let client = reqwest::blocking::Client::new();
    let response = client.post(token_url)
    .form(&params)
    .send();
   
    // Get the access token from the response
     let response_struct = response.unwrap().json::<models::Response>().unwrap();
     let access_token = response_struct.body.access_token.clone(); //TODO: fix using clone
     info!("Got Access Token: {}", access_token);
    
     //write_config(response_struct);
     access_token
}