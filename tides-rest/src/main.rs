// use tides_rest::stations::Station;
use tides_rest::measurements::Measurement;

fn main() -> Result<(), minreq::Error> {
    let data = Measurement::query_station_id(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 1)?;
    println!("{:#?}", data);

    Ok(())
}
