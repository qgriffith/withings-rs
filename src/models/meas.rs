//! # Measure model
//! Docs: https://developer.withings.com/oauth2/#operation/measure-getmeas
//! Example: https://developer.withings.com/oauth2/#section/Measure/Get-measure
//! Response body from the measure-getmeas endpoint

use serde::{Deserialize, Serialize};
use std::fmt;

/// Response from the API is a JSON object that includes the following fields:
#[derive(Serialize, Deserialize)]
pub struct ResponseMeas {
    pub status: i64,
    pub body: Body,
}

/// The body of the response is a JSON object that includes the following fields:
#[derive(Serialize, Deserialize)]
pub struct Body {
    updatetime: i64,
    timezone: String,
    pub measuregrps: Vec<Measuregrp>,
}

/// Struct collection of measures
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

/// Struct for each measure fields
#[derive(Serialize, Deserialize)]
pub struct Measure {
    pub value: i64,
    #[serde(rename = "type")]
    pub measure_type: i64,
    pub unit: i64,
    algo: i64,
    fm: i64,
}

/// CategoryType enum for the category field in the measure struct matches the values in the Withings API docs
pub enum CategoryType {
    Measures = 1,
    // User Measuers
    UserObjections = 2, // User Objectives
}

/// Implement Display for CategoryType enum so to_string() can be used to convert to the string value expected by the API
impl fmt::Display for CategoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CategoryType::Measures => write!(f, "1"),
            CategoryType::UserObjections => write!(f, "2"),
        }
    }
}

/// MeasureType enum for the measure type field in the measure struct matches the values in the Withings API docs
pub enum MeasureType {
    Weight = 1,
    // Weight (kg)
    Height = 4,
    // Height (meter)
    FatFreeMass = 5,
    // Fat Free Mass (kg)
    FatRatio = 6,
    // Fat Ratio (%)
    FatMassWeight = 8,
    // Fat Mass Weight (kg)
    DiastolicBloodPressure = 9, // Diastolic Blood Pressure (mmHg)
    SystolicBloodPressure = 10, // Systolic Blood Pressure (mmHg)
    HeartPulse = 11,
    // Heart Pulse (bpm)
    Temperature = 12,
    // Temperature (C)
    Sp02 = 54,
    // SpO2 (%)
    BodyTemperature = 71,
    // Body Temperature (C)
    SkinTemperature = 73,
    // Skin Temperature (C)
    MuscleMass = 76,
    // Muscle Mass (kg)
    Hydration = 77,
    // Hydration (%)
    BoneMass = 88,
    // Bone Mass (kg)
    PulseWaveVelocity = 91,
    // Pulse Wave Velocity (m/s)
    V02Max = 123,
    // VO2 max (mL/min/kg)
    AtrialFibrillation = 130,
    // Atrial Fibrillation (0 or 1)
    Qrs = 135,
    // QRS Duration (ms)
    VascularAge = 155,
    // Vascular Age (years)
    ExtracellularWater = 168,
    // Extracellular Water (kg)
    IntracellularWater = 169,
    // Intracellular Water (kg)
    VisceralFatMass = 170,
    // Visceral Fat Mass (kg)
    FatMass = 174,
    // Fat Mass (kg)
    MuscleMassSegments = 175, // Muscle Mass (kg)
}

/// Implement Display for MeasureType enum so to_string() can be used to convert to the string value expected by the API
impl fmt::Display for MeasureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MeasureType::Weight => write!(f, "1"),
            MeasureType::Height => write!(f, "4"),
            MeasureType::FatFreeMass => write!(f, "5"),
            MeasureType::FatRatio => write!(f, "6"),
            MeasureType::FatMassWeight => write!(f, "8"),
            MeasureType::DiastolicBloodPressure => write!(f, "9"),
            MeasureType::SystolicBloodPressure => write!(f, "10"),
            MeasureType::HeartPulse => write!(f, "11"),
            MeasureType::Temperature => write!(f, "12"),
            MeasureType::Sp02 => write!(f, "54"),
            MeasureType::BodyTemperature => write!(f, "71"),
            MeasureType::SkinTemperature => write!(f, "73"),
            MeasureType::MuscleMass => write!(f, "76"),
            MeasureType::Hydration => write!(f, "77"),
            MeasureType::BoneMass => write!(f, "88"),
            MeasureType::PulseWaveVelocity => write!(f, "91"),
            MeasureType::V02Max => write!(f, "123"),
            MeasureType::AtrialFibrillation => write!(f, "130"),
            MeasureType::Qrs => write!(f, "135"),
            MeasureType::VascularAge => write!(f, "155"),
            MeasureType::ExtracellularWater => write!(f, "168"),
            MeasureType::IntracellularWater => write!(f, "169"),
            MeasureType::VisceralFatMass => write!(f, "170"),
            MeasureType::FatMass => write!(f, "174"),
            MeasureType::MuscleMassSegments => write!(f, "175"),
        }
    }
}
