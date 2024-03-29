//! # measure.rs
//! Calls the withings API to get the list of measurements
//! https://developer.withings.com/oauth2/#operation/measure-getmeas

use crate::{api, models};
use log::{info, trace, warn};
use std::collections::HashMap;
use std::string::ToString;

/// Represents the parameters for a measurement request.
#[derive(Debug)]
pub struct MeasurementParams {
    pub access_token: String,
    pub client_id: String,
    pub meastype: String,
    pub category: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub offset: Option<String>,
    pub lastupdate: Option<String>,
}

/// Prepare parameters for measurement request.
///
/// This function takes a `MeasurementParams` struct as input and returns a `HashMap` of the prepared parameters for the request.
///
/// # Arguments
///
/// * `params` - A reference to a `MeasurementParams` struct.
///
/// # Returns
///
/// A `HashMap` containing the prepared parameters for the request.
fn prepare_params(params: &MeasurementParams) -> HashMap<&str, String> {
    let action = "getmeas".to_string();
    let mut map_params = HashMap::new();
    map_params.insert("client_id", params.client_id.clone());
    map_params.insert("action", action);
    map_params.insert("access_token", params.access_token.clone());
    map_params.insert("meastype", params.meastype.clone());
    map_params.insert("category", params.category.clone());
    if let Some(start) = &params.start {
        map_params.insert("startdate", start.clone());
    }
    if let Some(end) = &params.end {
        map_params.insert("enddate", end.clone());
    }
    if let Some(offset) = &params.offset {
        map_params.insert("offset", offset.clone());
    }
    if let Some(lastupdate) = &params.lastupdate {
        map_params.insert("lastupdate", lastupdate.clone());
    }
    map_params
}

/// Retrieves measurements from the API based on the provided parameters.
///
/// # Arguments
///
/// * `params` - The MeasurementParams struct containing the parameters for the API call.
///
/// # Returns
///
/// * `Result<models::meas::ResponseMeas, Box<dyn std::error::Error>>` - A Result enum containing either the measurements or an error.
///
/// # Documentation
/// https://developer.withings.com/api-reference/#tag/measure
pub fn get_measurements(
    params: &MeasurementParams,
) -> Result<models::meas::ResponseMeas, Box<dyn std::error::Error>> {
    // Prepare the parameters for the API call
    let map_params = prepare_params(params);
    trace!("Measure API parameters: {:?}", map_params);

    let client = reqwest::blocking::Client::new();
    let url = api::wapi_url("measure".to_string());
    let response = client.get(url).query(&map_params).send()?;

    // Check for errors from the API
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "API returned an error",
        )));
    }

    info!("Measure API response: {:?}", response);
    let measurements = response.json::<models::ResponseMeas>()?;

    Ok(measurements)
}
