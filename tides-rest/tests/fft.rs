use tides_signals::predictions;
use tides_rest;

#[test]
fn reconstruct_test() {
    let data = tides_rest::query_hours(
        "d3f822a0-e201-4a61-8913-589c74818ae0", 30)
        .unwrap();

    let signal = predictions::reconstruct(&data);

    let cmp = data.iter()
        .map(|mes| (mes.value, signal(mes.timestamp)));
    for (expected, actual) in cmp {
        assert!((actual - expected).abs() <= 0.1)
    }
}
