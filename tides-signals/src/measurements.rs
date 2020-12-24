use serde::Deserialize;
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
}
