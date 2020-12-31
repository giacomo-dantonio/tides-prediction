// use tides_rest::stations::Station;
use tides_rest;
// use tides_signals::measurements::Series;

// use tides_signals::predictions;
// use std::f32::consts::PI;

use std::fs::File;
use std::path::Path;
use std::io::Write;

fn main() -> Result<(), minreq::Error> {
    let data = tides_rest::query_hours(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 6)
        .unwrap();

    let mut buf = Vec::new();
    writeln!(&mut buf, "\"Timestamp\",\"Gauge\"")?;
    for mes in data {
        writeln!(&mut buf, "\"{}\",{}", mes.timestamp, mes.value)?;
    }

    let path = Path::new("measurements.csv");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(&buf) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // let series = Series::from_json(data.clone());
    // let minima = series.find_minimum(
    //     data.first().unwrap().timestamp.timestamp(),
    //     data.last().unwrap().timestamp.timestamp(),
    // );
    // for timestamp in minima {
    //     let local_values : Vec<f32> = (-20 .. 20)
    //         .map(|i| (timestamp + i * 60))
    //         .map(|ts| series.evaluate(ts))
    //         .collect();
    //     let value = series.evaluate(timestamp);
    //     for local_value in local_values {
    //         if value > local_value {
    //             println!("unexpected value ({}) > local_value({})", value, local_value);
    //         }
    //         // assert!(value <= local_value);
    //     }
    // }

    // let data = tides_rest::query_hours(
    //     "d3f822a0-e201-4a61-8913-589c74818ae0", 1)?;

    // println!("{:#?}", data);

    // let signal = predictions::reconstruct(&data);

    // let cmp : Vec<(f32, f32)> = data.iter()
    //     .map(|mes| (mes.value, signal(mes.timestamp)))
    //     .collect();
    // println!("{:#?}", cmp);

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
