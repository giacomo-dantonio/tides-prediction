use crate::measurements::Measurement;
use chrono::{DateTime, Utc};

extern crate web_sys;

fn parameter(measurements : &Vec<Measurement>) 
    -> impl Fn(DateTime<Utc>) -> f32 {

    let from = if measurements.len() > 0 {
        web_sys::console::log_1(&"Has measurements".into());
        measurements[0].timestamp
    }
    else {
        web_sys::console::log_1(&"Doesn't have measurements".into());
        Utc::now()
    };

    move |dt : DateTime<Utc>|
        (dt - from).num_hours() as f32
}

pub fn reconstruct(measurements : &Vec<Measurement>) -> impl Fn(DateTime<Utc>) -> f32
{
    web_sys::console::log_1(&"Collect samples".into());
    let samples : Vec<f32> = measurements.iter()
        .map(|mes| mes.value)
        .collect();

    web_sys::console::log_1(&"Create signal closure".into());
    let fun = crate::fft::reconstruct(&samples);

    web_sys::console::log_1(&"Create parameter closure".into());
    let par_fun = parameter(measurements);

    web_sys::console::log_1(&"Create combined closure".into());
    move |dt : DateTime<Utc>| fun(par_fun(dt))
}
