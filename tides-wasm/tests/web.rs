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
