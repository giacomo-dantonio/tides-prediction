use minreq;
use serde::Deserialize;
use chrono::{self, DateTime, Utc, Timelike};

static BASE_URL : &'static str =
    "https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations";

// Water measuraments
// https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations/d3f822a0-e201-4a61-8913-589c74818ae0/W/measurements?start=P30D
// https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations/4910050/W/measurements.png?start=P30D&width=1000&height=700

/// Represent a station measurement that can be queried on pegelonline.
#[derive(Deserialize, Debug)]
pub struct Measurement {
    pub timestamp: DateTime<Utc>,
    pub value: f32
}

impl Measurement {
    /// Query the API for the water measurements from
    /// the last days
    pub fn query_station_id(station_id : &str, days: u8)
        -> Result<Vec<Measurement>, minreq::Error>
    {
        let url = format!(
            "{}/{}/W/measurements?start=P{}D",
            BASE_URL, station_id, days);
        let response = minreq::get(url).send()?;
        let data : Vec<Measurement> = response.json()?;
        let hours_data : Vec<Measurement> = data
            .into_iter()
            .filter(|mes| mes.timestamp.minute() == 0)
            .collect();

        Ok(hours_data)
    }

    /// Query the API for the water measurements from
    /// the last 30 days
    pub fn query_station_id_all(station_id : &str)
    -> Result<Vec<Measurement>, minreq::Error>
    {
        Measurement::query_station_id(station_id, 30u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn query_station_id_test() {
        let data = Measurement::query_station_id(
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