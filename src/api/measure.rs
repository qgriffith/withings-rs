//! # measure.rs
//! Calls the withings API to get the list of measurements
//! https://developer.withings.com/oauth2/#operation/measure-getmeas

use crate::{api, models};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::string::ToString;

/// The measurement API takes a lot of arguments this struct sets those up to make
/// the call to the function cleaner
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

const ACTION: String = "getmeas".to_string();

/// get_measurements
/// Takes client_id, access_token, meastype, category, startdate, enddate, offset, lastupdate
/// meastype is the type of measurement to retrieve use enum MeasType
/// startdate, enddate, offset, lastupdate are optional
/// startdate, enddate, lastupdate are in the format of epoch time
/// Returns a Result of ResponseMeas
pub fn get_measurements(
    params: &MeasurementParams,
) -> Result<models::meas::ResponseMeas, Box<dyn std::error::Error>> {
    // Set up the parameters for the API call
    let mut map_params = HashMap::new();
    map_params.insert("client_id", &params.client_id);
    map_params.insert("action", &ACTION);
    map_params.insert("access_token", &params.access_token);
    map_params.insert("meastype", &params.meastype);
    map_params.insert("category", &params.category);
    if let Some(start) = &params.start {
        map_params.insert("startdate", start);
    }
    if let Some(end) = &params.end {
        map_params.insert("enddate", end);
    }
    if let Some(offset) = &params.offset {
        map_params.insert("offset", offset);
    }
    if let Some(lastupdate) = &params.lastupdate {
        map_params.insert("lastupdate", lastupdate);
    }

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
