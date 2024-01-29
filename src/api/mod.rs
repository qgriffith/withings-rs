pub mod auth;
pub mod measure;

pub fn wapi_url(path: String) -> String {
    format!("https://wbsapi.withings.net/{}", path)
}
