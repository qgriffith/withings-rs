//! # Withings API
//! Calls the withings API end points
//! Documentation: https://developer.withings.com/api-reference
pub mod auth;
pub mod measure;

/// wapi_url
/// Returns the URL for the withings API end point
pub fn wapi_url(path: String) -> String {
    format!("https://wbsapi.withings.net/{}", path)
}
