use crate::measurements::Measurement;
use chrono::{DateTime, Utc};

pub fn reconstruct(measurements : Vec<Measurement>) -> impl Fn(DateTime<Utc>) -> f32
{
    |_dt : DateTime<Utc>| 3.14f32
}