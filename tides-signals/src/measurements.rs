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

    /// Return a collection of timestamp, such that the interval between
    /// them contains a local minimum/maximum.
    fn find_bracket_points<T>(&self, from: i64, to: i64, resolution: i64, cmp: &T) -> Vec<(i64, i64)>
        where T: Fn(f32, f32) -> f32
    {
        let mut result = vec![];
        let start = from + (to - from) % resolution;
        let steps = (to - from) / resolution;



        for i in 0 .. steps {
            let lts = start + resolution * i;
            let mts = lts + resolution;
            let rts = lts + 2 * resolution;

            // FIXME: avoid evaluating each timestamp three times
            let lvalue = self.evaluate(lts);
            let mvalue = self.evaluate(mts);
            let rvalue = self.evaluate(rts);

            if cmp(mvalue, lvalue) == mvalue && cmp(mvalue, rvalue) == mvalue {
                result.push((lts, rts));
            }
        }

        result
    }

    pub fn find_extreme<T>(&self, from: i64, to: i64, cmp: &T) -> Vec<i64>
        where T : Fn(f32, f32) -> f32
    {
        // First we bracket all the local minima/maxima with intervals
        // of 2 hours length.
        let hour_brackets = self.find_bracket_points(from, to, 3600, cmp);
        hour_brackets.iter().map(|&(lb, up)| {
            // Then for each interval we look for the global minimum/maximum
            // inside the interval.
            // We do this by evaluating every minute.
            let (min_timestep, _min_val) = (lb .. up).step_by(60)
            .map(|ts| (ts, self.evaluate(ts)))
            .fold(
                (0, cmp(-1f32, 1f32) * -f32::INFINITY),
                |(acc_ts, acc_value), (ts, value)| {
                    if cmp(acc_value, value) == acc_value {
                        (acc_ts, acc_value)
                    }
                    else {
                        (ts, value)
                    }
                });
            min_timestep
        }).collect()
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
