use std::collections::HashMap;
use crate::models;
use log::{info, trace, warn};

pub fn get_measurements(access_token: &str, client_id: &str, meastype: &str, category: &str, 
    start: Option<&str>, 
    end: Option<&str>,
    offset: Option<&str>,
    lastupdate: Option<&str>)     
    -> Result<models::meas::ResponseMeas, Box<dyn std::error::Error>> {
    
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

    let client = reqwest::blocking::Client::new();
    let response = client.get("https://wbsapi.withings.net/measure")
    .query(&params)
    .send()?;


    let measurements = response.json::<models::ResponseMeas>()?;
    Ok(measurements)
}