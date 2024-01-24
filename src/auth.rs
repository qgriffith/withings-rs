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
    let access_token = response_struct.body.access_token;
    info!("Got Access Token: {}", access_token);
    
    // Return the access token
    access_token
  
}