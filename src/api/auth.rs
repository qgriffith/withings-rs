//! # auth.rs
//! This module handles the auth with the Withings API. It uses the client_id and client_secret
//! provided by Withings to auth with their API and get an access token. The access token is then
//! used to make requests to the Withings API. The access and refresh tokens are stored in a config
//!file for future use. The access token expires after 1 hour and the refresh token expires after
//! 1 year. The refresh token is used to get a new access token when the current access token expires.

use log::{info, trace, warn};
use random_string::generate;
use std::{collections::HashMap, env};

use crate::{
    api,
    api::config::{load_config, write_config},
    models, redirect,
};

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

pub fn refresh_token(
    client_id: String,
    client_secret: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = load_config()?;
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
