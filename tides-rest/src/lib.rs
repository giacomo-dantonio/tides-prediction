use minreq;
use chrono::{self, Timelike};
use tides_signals::measurements::Measurement;

pub mod stations;

static BASE_URL : &'static str =
    "https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations";

// Water measuraments
// https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations/d3f822a0-e201-4a61-8913-589c74818ae0/W/measurements?start=P30D
// https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations/4910050/W/measurements.png?start=P30D&width=1000&height=700

/// Query the API for the water measurements from
/// the last days.
/// The data series will contain one value per minute.
pub fn query(station_id : &str, days: u8)
    -> Result<Vec<Measurement>, minreq::Error>
{
    let url = format!(
        "{}/{}/W/measurements?start=P{}D",
        BASE_URL, station_id, days);
    let response = minreq::get(url).send()?;
    response.json()
}

/// Query the API for the water measurements from
/// the last 30 days
/// The data series will contain one value per minute.
pub fn query_all(station_id : &str)
-> Result<Vec<Measurement>, minreq::Error>
{
    query(station_id, 30u8)
}

/// Query the API for the water measurements from
/// the last days
/// The data series will contain one value per hour.
pub fn query_hours(station_id : &str, days: u8)
    -> Result<Vec<Measurement>, minreq::Error>
{
    let data = query(station_id, days)?;
    let hours_data : Vec<Measurement> = data
        .into_iter()
        .filter(|mes| mes.timestamp.minute() == 0)
        .collect();

    Ok(hours_data)
}

/// Query the API for the water measurements from
/// the last 30 days
/// The data series will contain one value per hour.
pub fn query_hours_all(station_id : &str)
-> Result<Vec<Measurement>, minreq::Error>
{
    query_hours(station_id, 30u8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn query_station_id_test() {
        let data = query(
            "d3f822a0-e201-4a61-8913-589c74818ae0", 1);
        
        assert!(data.is_ok());

        let measurements = data.unwrap();
        assert!(measurements.len() > 0);
        
        let yesterday = Utc::today() - Duration::days(1);
        assert!(measurements
            .iter()
            .all(|mes| mes.timestamp.date() >= yesterday));
    }
}