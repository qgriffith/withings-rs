// Docs: https://developer.withings.com/oauth2/#operation/oauth2-token
// Example: https://developer.withings.com/oauth2/#section/Authentication/Obtaining-an-access-token
// Response body from the OAuth2 token endpoint

use serde::{Serialize, Deserialize};

// Response from the API is a JSON object that includes the following fields:
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub body: Body,
    pub status: i64,
}

// The body of the response is a JSON object that includes the following fields:
#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
    pub token_type: String,
    pub userid: String,
}