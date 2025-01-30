//! # measure.rs
//! Calls the Withings API to get the list of measurements
//! https://developer.withings.com/oauth2/#operation/measure-getmeas

use crate::{api, models};
use log::{info, trace, warn};
use std::collections::HashMap;
use std::{error::Error, io};

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

impl MeasurementParams {
    /// Converts the `MeasurementParams` struct into a `HashMap` of request parameters.
    pub fn to_query_params(&self) -> HashMap<&str, String> {
        let mut params = HashMap::new();

        params.insert("client_id", self.client_id.clone());
        params.insert("action", "getmeas".to_string());
        params.insert("access_token", self.access_token.clone());
        params.insert("meastype", self.meastype.clone());
        params.insert("category", self.category.clone());

        // Add optional parameters if provided.
        if let Some(start) = &self.start {
            params.insert("startdate", start.clone());
        }
        if let Some(end) = &self.end {
            params.insert("enddate", end.clone());
        }
        if let Some(offset) = &self.offset {
            params.insert("offset", offset.clone());
        }
        if let Some(lastupdate) = &self.lastupdate {
            params.insert("lastupdate", lastupdate.clone());
        }

        params
    }
}

/// Retrieves measurements from the Withings API based on the provided parameters.
///
/// # Arguments
///
/// * `params` - The `MeasurementParams` struct containing the parameters for the API call.
///
/// # Returns
///
/// Returns a `Result` with either `models::meas::ResponseMeas` or an error.
///
/// # Documentation
/// https://developer.withings.com/api-reference/#tag/measure
pub fn get_measurements(
    params: &MeasurementParams,
) -> Result<models::meas::ResponseMeas, Box<dyn Error>> {
    // Step 1: Prepare the parameters for the API call
    let query_params = params.to_query_params();
    trace!("Measure API query parameters: {:?}", query_params);

    // Step 2: Prepare the API call
    let client = reqwest::blocking::Client::new();
    let url = api::wapi_url("measure".to_string());

    // Step 3: Make the API request
    let response = client.get(&url).query(&query_params).send()?;

    // Step 4: Handle response errors
    if response.status().is_client_error() || response.status().is_server_error() {
        warn!("Error response from the API: {:?}", response);
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("API returned an error: {}", response.status()),
        )));
    }

    // Step 5: Parse the JSON response
    info!("Successful response from Measure API: {:?}", response);
    response.json::<models::meas::ResponseMeas>().map_err(|e| {
        // Convert serde JSON parsing errors into a compatible error
        warn!("Failed to parse API response: {:?}", e);
        Box::new(e) as Box<dyn Error>
    })
}
