use tides_signals::predictions;
use tides_signals::measurements::Series;
use tides_rest;

#[test]
fn reconstruct_test() {
    let data = tides_rest::query(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 2)
        .unwrap();

    let signal = predictions::reconstruct(&data);

    let cmp = data.iter()
        .map(|mes| (mes.value, signal(mes.timestamp)));
    for (expected, actual) in cmp {
        assert!((actual - expected).abs() <= 0.1)
    }
}

#[test]
fn extremes_test() {
    let data = tides_rest::query(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 6)
        .unwrap();

    let series = Series::from_json(data.clone());
    let minima = series.find_minimum(
        data.first().unwrap().timestamp.timestamp(),
        data.last().unwrap().timestamp.timestamp(),
    );

    assert!(minima.len() > 0);
    for timestamp in minima {
        let local_values = (-20 .. 20)
            .map(|i| (timestamp + i * 60))
            .map(|ts| series.evaluate(ts));
        let value = series.evaluate(timestamp);
        for local_value in local_values {
            assert!(value <= local_value);
        }
    }
}
