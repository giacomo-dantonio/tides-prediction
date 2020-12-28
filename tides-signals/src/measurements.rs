use serde::Deserialize;
use chrono::{self, DateTime, NaiveDateTime, Utc};
use wasm_bindgen::prelude::*;
use crate::predictions;

/// Represent a station measurement that can be queried on pegelonline.
#[derive(Deserialize, Debug, Clone)]
pub struct Measurement {
    pub timestamp: DateTime<Utc>,
    pub value: f32
}

#[wasm_bindgen]
pub struct Series {
    measurements: Vec<Measurement>,
    signal: Box<dyn Fn(DateTime<Utc>) -> f32>
}

impl Series {
    pub fn from_json(measurements: Vec<Measurement>) -> Series {
        let signal = Box::new(predictions::reconstruct(&measurements)); 
        Series {
            measurements,
            signal
        }
    }

    fn find_bracket_points<T>(&self, from: i64, to: i64, resolution: i64, cmp: &T) -> Vec<i64>
        where T: Fn(f32, f32) -> f32
    {
        let start = from + (to - from) % resolution;
        let steps = (to - from) / resolution;

        (0 .. steps)
            .map(|i| start + resolution * i)
            .filter(|&ts| self.is_extreme(ts, resolution, cmp))
            .collect()
    }

    fn is_extreme<T>(&self, timestamp: i64, resolution: i64, cmp: &T) -> bool
        where T : Fn(f32, f32) -> f32
    {
        let (from, to) = (-3, 3);
        let local_values = (from .. to)
            .map(|i| (timestamp + i * resolution))
            .map(|ts| self.evaluate(ts));
        let value = self.evaluate(timestamp);
        let initial = local_values.clone().next().unwrap_or(0f32);
        let minmax = local_values.fold(initial, cmp);

        value == minmax
    }

    pub fn find_extreme<T>(&self, from: i64, to: i64, cmp: &T) -> Vec<i64>
        where T : Fn(f32, f32) -> f32
    {
        // We start by checking only the hours values and
        // then refine the search until we reach minutes.
        // But if to and from are closer than 10 hours apart,
        // we will use a different scale.

        // 1 hour = 3600
        // but the resolution here needs to be a power of 2 * 60
        let mut resolution = 3840.min((to - from) / 10);
        let mut extremes = self.find_bracket_points(from, to, resolution, cmp);

        while resolution >= 60 {
            extremes = extremes
            .iter()
            .map(|ts|
                self.find_bracket_points(
                    ts - resolution,
                    ts + resolution,
                    resolution / 2,
                    cmp
                )
            )
            .flatten()
            .collect();
            resolution = resolution / 2
        }
        extremes
    }
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

    pub fn batch_evaluate(&self, timestamps: &[i64]) -> Vec<f32> {
        timestamps.iter()
            .map(|&timestamp| self.evaluate(timestamp))
            .collect()
    }

    pub fn find_minimum(&self, from: i64, to: i64) -> Vec<i64> {
        let cmp = |lhs: f32, rhs: f32| lhs.min(rhs);
        self.find_extreme(from, to, &cmp)
    }

    pub fn find_maximum(&self, from: i64, to: i64) -> Vec<i64> {
        let cmp = |lhs: f32, rhs: f32| lhs.max(rhs);
        self.find_extreme(from, to, &cmp)
    }
}
