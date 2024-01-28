use std::fmt;

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

pub enum MeasureType {
    Weight = 1,
    Height = 4,
    FatFreeMass = 5,
    FatRatio = 6,
    FatMassWeight = 8,
    DiastolicBloodPressure = 9,
    SystolicBloodPressure = 10,
    HeartPulse = 11,
    Temperature = 12,
    Sp02 = 54,
    BodyTemperature = 71,
    SkinTemperature = 73,
    MuscleMass = 76,
    Hydration = 77,
    BoneMass = 88,
    PulseWaveVelocity = 91,
    V02Max = 123,
    AtrialFibrillation = 130,
    Qrs = 135,
    VascularAge = 155,
    ExtracellularWater = 168,
    IntracellularWater= 169,
    VisceralFatMass = 170,
    FatMass = 174,
    MuscleMassSegments = 175
}

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