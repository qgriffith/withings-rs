//! # auth.rs
//! This module handles the auth with the Withings API. It uses the client_id and client_secret
//! provided by Withings to auth with their API and get an access token. The access token is then
//! used to make requests to the Withings API. The access and refresh tokens are stored in a config
//!file for future use. The access token expires after 1 hour and the refresh token expires after
//! 1 year. The refresh token is used to get a new access token when the current access token expires.

use log::{info, trace, warn};
use random_string::generate;
use std::{collections::HashMap, env};

use crate::{api, models, redirect};

const AUTH_URL: &str = "https://account.withings.com/oauth2_user/authorize2";
const REDIRECT_URL: &str = "http://localhost:8888";
const API_SCOPE: &str = "user.info,user.metrics,user.activity";
const CSRF_CHARSET: &str = "ABCDEfghiJKLnmoQRStuvWxyZ1234567890";
const ACTION: &str = "requesttoken";

/// This struct represents the parameters required for making token-related API requests.
///
/// # Fields
///
/// * `client_id`: The client ID generated for your application.
///
/// * `client_secret`: The client secret generated for your application.
///
/// * `grant_type`: The grant type used for authentication.
///
/// * `redirect_uri`: The redirect URI to redirect the user after authenticating (optional).
///
/// * `code`: The authorization code obtained from the authentication process (optional).
///
/// * `refresh_token`: The refresh token obtained from a previous authentication (optional).
#[derive(Default)]
struct TokenParams {
    client_id: String,
    client_secret: String,
    grant_type: String,
    redirect_uri: Option<String>,
    code: Option<String>,
    refresh_token: Option<String>,
}

/// Retrieves the path to the configuration file.
///
/// The path to the configuration file is determined by the value of the `WITHINGS_CONFIG_FILE`
/// environment variable. If the variable is not set, the default file path `config.json` is used.
///
/// # Example
///
/// ```
/// use std::env;
/// use log::info;
///
/// pub fn get_config_file() -> String {
///     let config_file =
///         env::var("WITHINGS_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
///     info!("Using config file: {}", config_file);
///     config_file
/// }
/// ```
///
/// # Returns
///
/// The path to the configuration file as a `String`.
pub fn get_config_file() -> String {
    let config_file =
        env::var("WITHINGS_CONFIG_FILE").unwrap_or_else(|_| "config.json".to_string());
    info!("Using config file: {}", config_file);
    config_file
}

/// Prepare token params for making a request to obtain a token.
///
/// # Arguments
///
/// * `token_params` - A struct containing the necessary parameters for requesting a token.
///
/// # Returns
///
/// A hashmap containing the token parameters required for making the request.
fn prepare_token_params(token_params: TokenParams) -> HashMap<&'static str, String> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert("client_id", token_params.client_id);
    params.insert("client_secret", token_params.client_secret);
    params.insert("grant_type", token_params.grant_type);

    if let Some(redirect_uri) = token_params.redirect_uri {
        params.insert("redirect_uri", redirect_uri);
    }

    if let Some(code) = token_params.code {
        params.insert("code", code);
    }

    if let Some(refresh_token) = token_params.refresh_token {
        params.insert("refresh_token", refresh_token);
    }

    params.insert("action", ACTION.to_string());
    params
}

/// Retrieves the access code for Withings API.
///
/// This function takes in the `client_id` and `client_secret` as arguments and returns the access code as a `String`.
///
/// # Arguments
///
/// * `client_id` - A `String` representing the client ID provided by Withings.
/// * `client_secret` - A `String` representing the client secret provided by Withings.
///
/// # Returns
///
/// * If the access code is retrieved successfully, it returns a `Result` with the access code as a `String`.
/// * If any error occurs during the retrieval process, it returns a `Result` with a `Box<dyn Error>` representing the error.
pub fn get_access_code(
    client_id: String,
    client_secret: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let auth_url = build_auth_url(&client_id, AUTH_URL, API_SCOPE, REDIRECT_URL)?;
    println!("Browse to: {}\n", auth_url);

    let auth_response = redirect::server::run();
    let auth_code = auth_response["code"].to_string();
    info!("Got Auth Code: {}", auth_code);

    check_csrf_token(&auth_response["state"], &auth_url)?;
    let token_params = TokenParams {
        client_id,
        client_secret,
        grant_type: "authorization_code".to_string(),
        redirect_uri: Some(REDIRECT_URL.to_string()),
        code: Some(auth_code),
        ..Default::default()
    };

    request_access_token(token_params)
}

/// Writes the configuration to a file in JSON format.
///
/// # Arguments
///
/// * `access_token` - The access token to be written to the configuration file.
/// * `refresh_token` - The refresh token to be written to the configuration file.
///
/// # Panics
///
/// This function panics if it encounters any errors during file creation or writing.
fn write_config(access_token: &String, refresh_token: &String) {
    let config = models::Config {
        access_token: access_token.to_string(),
        refresh_token: refresh_token.to_string(),
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

/// Loads the configuration from a JSON file.
///
/// # Panics
///
/// This function will panic if the configuration file cannot be opened or read.
/// It will also panic if there is an error while deserializing the JSON data into
/// the `Config` struct.
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

/// Refreshes the access token using the provided client ID and client secret.
///
/// # Arguments
///
/// * `client_id` - The client ID.
/// * `client_secret` - The client secret.
///
/// # Returns
///
/// * `Result<String, Box<dyn std::error::Error>>` - A `Result` containing either the access token as a `String` on success, or a boxed `std::error::Error
pub fn refresh_token(
    client_id: String,
    client_secret: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = load_config();
    let grant_type = "refresh_token".to_string();
    let refresh_token = config.refresh_token;

    let token_struct = TokenParams {
        client_id,
        client_secret,
        grant_type,
        redirect_uri: None,
        code: None,
        refresh_token: Some(refresh_token),
    };

    let params = prepare_token_params(token_struct);

    trace!("Refresh Token API parameters: {:?}", params);

    // Make the refresh token request
    let token_url = api::wapi_url("v2/oauth2/".to_string());
    let client = reqwest::blocking::Client::new();
    let response = client.post(token_url).form(&params).send();

    if response.is_err() {
        warn!("Refresh API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "API returned an error",
        )));
    }
    info!("Refresh Response: {:?}", response);

    // Get the access token from the response
    let response_struct = response
        .unwrap()
        .json::<models::OauthResponse>()
        .unwrap_or_else(|e| {
            panic!("Error: {}", e);
        });
    let access_token = response_struct.body.access_token;
    let refresh_token = response_struct.body.refresh_token;
    info!("Got Access Token: {}", access_token);

    write_config(&access_token, &refresh_token);
    Ok(access_token)
}

fn build_auth_url(
    client_id: &str,
    auth_url_base: &str,
    scope: &str,
    redirect_uri: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let state = generate(12, CSRF_CHARSET);
    Ok(format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
        auth_url_base, client_id, redirect_uri, scope, state
    ))
}

fn check_csrf_token(state: &str, expected_state: &str) -> Result<(), Box<dyn std::error::Error>> {
    if state != expected_state {
        warn!("CSRF token mismatch!");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "CSRF token mismatch!",
        )));
    }
    Ok(())
}

fn request_access_token(params: TokenParams) -> Result<String, Box<dyn std::error::Error>> {
    let token_url = api::wapi_url("v2/oauth2/".to_string());
    let params_map = prepare_token_params(params);
    trace!("Auth API parameters: {:?}", params_map);

    let response = reqwest::blocking::Client::new()
        .post(token_url)
        .form(&params_map)
        .send()?;

    let response_struct: models::OauthResponse = response.json()?;
    let access_token = response_struct.body.access_token;
    let refresh_token = response_struct.body.refresh_token;

    info!("Got Access Token: {}", access_token);
    write_config(&access_token, &refresh_token);

    Ok(access_token)
}
