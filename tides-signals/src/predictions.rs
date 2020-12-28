use crate::measurements::Measurement;
use chrono::{DateTime, Utc};

fn parameter(measurements : &Vec<Measurement>) 
    -> impl Fn(DateTime<Utc>) -> f32 {

    let from = if measurements.len() > 0 {
        measurements[0].timestamp
    }
    else {
        Utc::now()
    };

    move |dt : DateTime<Utc>|
        (dt - from).num_minutes() as f32
}

pub fn reconstruct(measurements : &Vec<Measurement>) -> impl Fn(DateTime<Utc>) -> f32
{
    let samples : Vec<f32> = measurements.iter()
        .map(|mes| mes.value)
        .collect();

    let fun = crate::fft::reconstruct(&samples);

    let par_fun = parameter(measurements);

    move |dt : DateTime<Utc>| fun(par_fun(dt))
}
