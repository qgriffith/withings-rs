//! Auth Models
//! Docs: https://developer.withings.com/oauth2/#operation/oauth2-token
//! Example: https://developer.withings.com/oauth2/#section/Authentication/Obtaining-an-access-token
//! Response body from the OAuth2 token endpoint

use serde::{Deserialize, Serialize};

/// Response from the Oauth API is a JSON object that includes the following fields:
#[derive(Debug, Serialize, Deserialize)]
pub struct OauthResponse {
    pub body: Auth,
    pub status: i64,
}

/// The JSON fields from the Oauth response flow:
#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    #[serde(skip)]
    //Skip this field because it changes from string to int depending on the response. It isn't needed for the app since we have one user
    pub userid: String,
}

/// Config file struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub access_token: String,
    pub refresh_token: String,
}
