use js_sys;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::generator::DefaultGenerator;
use crate::model::Board;
use crate::ui::{init_generator, place_words};

pub fn new_board(size: u32) -> JsValue {
    JsValue::from_serde(&Board::new(size as usize)).unwrap()
}

pub fn init_gen(words_json: js_sys::Array) -> JsValue {
    let words: Vec<String> = JsValue::into_serde(&JsValue::from(words_json)).unwrap();
    JsValue::from_serde(&init_generator(&words)).unwrap()
}

pub fn fill_slots(board: &JsValue, n_words: u32, gen: &JsValue) -> JsValue {
    let mut b: Board = JsValue::into_serde(board).unwrap();
    let generator: DefaultGenerator = JsValue::into_serde(gen).unwrap();

    place_words(&mut b, n_words as usize, &generator);

    b.fill_blank_slots(&generator);

    JsValue::from_serde(&b).unwrap()
}

#[wasm_bindgen]
pub fn run(words_json: js_sys::Array, size: u32, n_words: u32) -> JsValue {
    let board = new_board(size);
    let b = fill_slots(&board, n_words, &init_gen(words_json));
    b
}