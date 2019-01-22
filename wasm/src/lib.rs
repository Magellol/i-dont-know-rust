extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// Used for typing functions coming from JavaScript.
// extern {}

#[wasm_bindgen]
pub fn sum(list: Vec<u32>) -> u32 {
  list.iter().fold(0, |acc, val| acc + val)
}

#[wasm_bindgen]
// This will throw an error in JS if called.
pub fn will_throw() -> Result<String, JsValue> {
  Err(JsValue::from("This is an error from rust"))
}
