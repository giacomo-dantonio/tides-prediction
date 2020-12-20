use tides_rest::stations::Station;

fn main() -> Result<(), minreq::Error> {
    let stations = Station::query();
    println!("{:?}", stations);

    Ok(())
}
