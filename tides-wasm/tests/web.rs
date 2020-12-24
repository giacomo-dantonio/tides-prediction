//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use tides_signals::measurements::Measurement;
use tides_wasm::utils;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    utils::set_panic_hook();

    let data = Measurement::get_number();
    assert_eq!(data, 42);
}


#[wasm_bindgen_test]
fn test_json() {
    utils::set_panic_hook();

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
    let data = Measurement::from_json(json_str);
    assert_eq!(data, vec![393f32, 466f32]);
}
