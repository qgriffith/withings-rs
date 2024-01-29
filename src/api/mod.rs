pub mod auth;
pub mod measure;

// Returns the URL for the withings API
pub fn wapi_url(path: String) -> String {
    format!("https://wbsapi.withings.net/{}", path)
}
