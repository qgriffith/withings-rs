use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseMeas {
   pub status: i64,
   pub body: Body,
}

#[derive(Serialize, Deserialize)]
pub struct Body {
    updatetime: i64,
    timezone: String,
    pub measuregrps: Vec<Measuregrp>,
}

#[derive(Serialize, Deserialize)]
pub struct Measuregrp {
    grpid: i64,
    attrib: i64,
    date: i64,
    created: i64,
    modified: i64,
    category: i64,
    deviceid: String,
    hash_deviceid: String,
    pub measures: Vec<Measure>,
    modelid: i64,
    model: String,
    comment: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Measure {
   pub value: i64,
    #[serde(rename = "type")]
   pub measure_type: i64,
   pub  unit: i64,
    algo: i64,
    fm: i64,
}