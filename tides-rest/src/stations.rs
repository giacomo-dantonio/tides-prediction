use minreq;
use serde::Deserialize;

static BASE_URL : &'static str =
    "https://www.pegelonline.wsv.de/webservices/rest-api/v2/stations";

/// Represent a station that can be queried on pegelonline.
#[derive(Deserialize, Debug)]
pub struct Station {
    pub uuid: String,
    pub number: String,
    pub shortname: String,
    pub longname: String,
}

impl Station 
{
    /// Query for all available stations.
    ///
    /// ```
    /// use tides_rest::stations::Station;
    /// let stations = Station::query();
    /// assert!(stations.is_ok());
    ///
    /// assert!(stations
    ///     .unwrap()
    ///     .iter()
    ///     .find(|&station| station.shortname == "BHV ALTER LEUCHTTURM")
    ///     .is_some());
    /// ```
    pub fn query() -> Result<Vec<Station>, minreq::Error> {
        let response = minreq::get(BASE_URL).send()?;
        response.json::<Vec<Station>>()
    }

    /// Query for stations at a given location on earth
    /// ```
    /// use tides_rest::stations::Station;
    ///
    /// let bhv_latitude = 53.55021;
    /// let bhv_longitude = 8.57673;
    /// let stations = Station::query_location(bhv_latitude, bhv_longitude, 10.0);
    /// assert!(stations.is_ok());
    ///
    /// assert!(stations
    ///     .unwrap()
    ///     .iter()
    ///     .find(|&station| station.shortname == "BHV ALTER LEUCHTTURM")
    ///     .is_some());
    /// ```
    pub fn query_location(latitude : f32, longitude : f32, radius : f32) -> Result<Vec<Station>, minreq::Error> {
        let url = format!(
            "{}?latitude={}&longitude={}&radius={}",
            BASE_URL, latitude, longitude, radius);
        let response = minreq::get(url).send()?;
        response.json()
    }
}
