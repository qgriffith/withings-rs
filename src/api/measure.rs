use std::collections::HashMap;
use crate::{models, api};
use log::{info, trace, warn};

// Calls the withings API to get the list of measurements
// https://developer.withings.com/oauth2/#operation/measure-getmeas
// Takes client_id, access_token, meastype, category, startdate, enddate, offset, lastupdate
// meastype is the type of measurement to retrieve use enum MeasType
// startdate, enddate, offset, lastupdate are optional
// startdate, enddate, lastupdate are in the format of epoch time
// Returns a Result of ResponseMeas
// TODO: So many arguments there has to be a cleaner way to do this

pub fn get_measurements(access_token: &str, client_id: &str, meastype: &str, category: &str, 
    start: Option<&str>, 
    end: Option<&str>,
    offset: Option<&str>,
    lastupdate: Option<&str>)     
    -> Result<models::meas::ResponseMeas, Box<dyn std::error::Error>> {
    
    // Set up the parameters for the API call
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("action", "getmeas");
    params.insert("access_token", access_token);
    params.insert("meastype", meastype);
    params.insert("category", category);
    if start.is_some() {
        params.insert("startdate", start.unwrap());
    }
    if end.is_some() {
        params.insert("enddate", &end.unwrap());
    }
    if offset.is_some() {
        params.insert("offset", &offset.unwrap());
    }
    if lastupdate.is_some() {
        params.insert("lastupdate", &lastupdate.unwrap());
    }

    trace!("API parameters: {:?}", params);

    let client = reqwest::blocking::Client::new();
    let url = api::wapi_url("measure".to_string());
    let response = client.get(&url)
    .query(&params)
    .send()?;
    
    // Check for errors from the API
    if response.status().is_client_error() {
        warn!("API response: {:?}", response);
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "API returned an error")));
    }

    info!("API response: {:?}", response);
    
    let measurements = response.json::<models::ResponseMeas>()?;
    Ok(measurements)
}