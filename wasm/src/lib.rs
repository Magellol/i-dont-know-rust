extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
}

#[wasm_bindgen]
pub fn sum(list: Vec<u32>) -> u32 {
    list.iter().fold(0, |acc, val| acc + val)
}