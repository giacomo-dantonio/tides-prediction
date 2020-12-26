use serde::Deserialize;
use chrono::{self, DateTime, NaiveDateTime, Utc};
use wasm_bindgen::prelude::*;
use crate::predictions;

/// Represent a station measurement that can be queried on pegelonline.
#[derive(Deserialize, Debug)]
pub struct Measurement {
    pub timestamp: DateTime<Utc>,
    pub value: f32
}

#[wasm_bindgen]
pub struct Series {
    measurements: Vec<Measurement>,
    signal: Box<dyn Fn(DateTime<Utc>) -> f32>
}

#[wasm_bindgen]
impl Series {
    pub fn from_data(timestamps: &[i64], values: &[f32]) -> Series {
        let measurements = timestamps.iter().zip(values)
            .map(|(&ts, &val)| Measurement {
                timestamp: DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp(ts, 0), Utc),
                value: val
            })
            .collect();

        let signal = Box::new(predictions::reconstruct(&measurements)); 
        Series {
            measurements,
            signal
        }
    }

    pub fn times(&self) -> Vec<i64> {
        self.measurements
            .iter()
            .map(|mes| mes.timestamp.timestamp())
            .collect()
    }

    pub fn values(&self) -> Vec<f32> {
        self.measurements
            .iter()
            .map(|mes| mes.value)
            .collect()
    }

    pub fn evaluate(&self, timestamp: i64) -> f32 {
        let dt = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        (self.signal)(dt)
    }
}
