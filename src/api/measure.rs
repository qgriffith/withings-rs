//! # measure.rs
//! Calls the withings API to get the list of measurements
//! https://developer.withings.com/oauth2/#operation/measure-getmeas

use crate::{api, models};
use log::{info, trace, warn};
use std::collections::HashMap;

/// get_measurements
/// Takes client_id, access_token, meastype, category, startdate, enddate, offset, lastupdate
/// meastype is the type of measurement to retrieve use enum MeasType
/// startdate, enddate, offset, lastupdate are optional
/// startdate, enddate, lastupdate are in the format of epoch time
/// Returns a Result of ResponseMeas
#[allow(clippy::too_many_arguments)]
pub fn get_measurements(
    access_token: &str,
    client_id: &str,
    meastype: &str,
    category: &str,
    start: Option<&str>,
    end: Option<&str>,
    offset: Option<&str>,
    lastupdate: Option<&str>,
) -> Result<models::meas::ResponseMeas, Box<dyn std::error::Error>> {
    // Set up the parameters for the API call
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("action", "getmeas");
    params.insert("access_token", access_token);
    params.insert("meastype", meastype);
    params.insert("category", category);
    if let Some(start) = start {
        params.insert("startdate", start);
    }
    if let Some(end) = end {
        params.insert("enddate", end);
    }
    if let Some(offset) = offset {
        params.insert("offset", offset);
    }
    if let Some(lastupdate) = lastupdate {
        params.insert("lastupdate", lastupdate);
    }

    trace!("Measure API parameters: {:?}", params);

    let client = reqwest::blocking::Client::new();
    let url = api::wapi_url("measure".to_string());
    let response = client.get(url).query(&params).send()?;

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
