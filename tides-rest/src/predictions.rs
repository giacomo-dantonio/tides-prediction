use crate::measurements::Measurement;
use chrono::{DateTime, Utc};
use tides_signals;

fn parameter(measurements : &Vec<Measurement>) 
    -> impl Fn(DateTime<Utc>) -> f32 {
    let from = if let Some(mes) = measurements.first() {
        mes.timestamp
    }
    else {
        Utc::now()
    };

    move |dt : DateTime<Utc>|
        (dt - from).num_hours() as f32
}

pub fn reconstruct(measurements : &Vec<Measurement>) -> impl Fn(DateTime<Utc>) -> f32
{
    let samples : Vec<f32> = measurements.iter()
        .map(|mes| mes.value)
        .collect();
    let fun = tides_signals::reconstruct(&samples);
    let par_fun = parameter(measurements);

    move |dt : DateTime<Utc>| fun(par_fun(dt))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_test() {
        let measurements : Vec<Measurement> = Measurement::query_station_id_all(
            "d3f822a0-e201-4a61-8913-589c74818ae0")
            .unwrap();

        let par_fun = parameter(&measurements);
        let from = measurements.first().unwrap().timestamp;

        let cmp = measurements.iter()
        .map(|mes| (
            mes.timestamp,
            from + chrono::Duration::hours(par_fun(mes.timestamp) as i64)
        ));

        for (expected, actual) in cmp {
            assert_eq!(expected, actual);
        }
    }
}