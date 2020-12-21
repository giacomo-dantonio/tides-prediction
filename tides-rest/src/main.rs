// use tides_rest::stations::Station;
// use tides_rest::measurements::Measurement;
use tides_signals;
use std::f32::consts::PI;

fn main() -> Result<(), minreq::Error> {
    // let data = Measurement::query_station_id(
    //     "d3f822a0-e201-4a61-8913-589c74818ae0", 1)?;
    // println!("{:#?}", data);

    let fun = |t: f32| (PI * t / 20f32).cos() + (1.5 * PI * t / 20f32).cos() + 3.14;
    let samples : Vec<f32> = (0 .. 40).map(|i| fun(i as f32)).collect();

    let signal = tides_signals::reconstruct(&samples);
    
    let cmp : Vec<(f32, f32)> = (0 .. 40)
        .map(|i| (fun(i as f32), signal(i as f32)))
        .collect();
    println!("{:#?}", cmp);
    Ok(())
}
