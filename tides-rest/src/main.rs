// use tides_rest::stations::Station;
use tides_rest::measurements::Measurement;
use tides_rest::predictions;
// use std::f32::consts::PI;

fn main() -> Result<(), minreq::Error> {
    let data = Measurement::query_hours(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 30)?;

    let signal = predictions::reconstruct(&data);

    let cmp : Vec<(f32, f32)> = data.iter()
        .map(|mes| (mes.value, signal(mes.timestamp)))
        .collect();
    println!("{:#?}", cmp);

    // println!("{:#?}", data);

    // let fun = |t: f32| (PI * t / 20f32).cos() + (1.5 * PI * t / 20f32).cos() + 3.14;
    // let samples : Vec<f32> = (0 .. 40).map(|i| fun(i as f32)).collect();

    // let signal = tides_signals::reconstruct(&samples);
    
    // let cmp : Vec<(f32, f32)> = (0 .. 40)
    //     .map(|i| (fun(i as f32), signal(i as f32)))
    //     .collect();
    // println!("{:#?}", cmp);
    Ok(())
}
