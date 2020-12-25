//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use tides_signals;
use tides_signals::measurements::Series;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_json() {
    tides_signals::set_panic_hook();

    let json_str = "[
        {
            \"timestamp\": \"2020-12-23T14:00:00Z\",
            \"value\": 393
        },
        {
            \"timestamp\": \"2020-12-23T15:00:00Z\",
            \"value\": 466
        }
    ]";
    let series = Series::from_json(json_str);
    let times = series.times();
    let values = series.values();

    for (timestamp, value) in times.into_iter().zip(values) {
        let prediction = series.evaluate(timestamp);
        assert!((prediction - value).abs() <= 0.1);
    }
}
