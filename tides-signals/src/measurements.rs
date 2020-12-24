use serde::Deserialize;
use serde_json;
use chrono::{self, DateTime, Utc};
use wasm_bindgen::prelude::*;

/// Represent a station measurement that can be queried on pegelonline.
#[derive(Deserialize, Debug)]
pub struct Measurement {
    pub timestamp: DateTime<Utc>,
    pub value: f32
}

#[wasm_bindgen]
impl Measurement {
    pub fn get_number() -> u32 {
        42
    }

    pub fn from_json(json_str: &str) -> Vec<f32> {
        let data : Vec<Measurement> = serde_json::from_str(json_str)
            .unwrap_or(vec![]);
        data.iter()
            .map(|mes| mes.value)
            .collect()
    }
}
