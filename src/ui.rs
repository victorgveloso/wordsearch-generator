use std::io::stdin;

use js_sys::*;
use wasm_bindgen::prelude::*;

use crate::generator::*;
use crate::model::{Board, Word, WordFactory};

pub fn place_words(board: &mut Board, words: Vec<&str>, gen: &DefaultGenerator) {
    for _ in 0..words.len() {
        let w = generate_word(&mut gen.clone(), &board);

        println!("{}ª linha, {}ª coluna", w.start_pos.0 + 1, w.start_pos.1 + 1);

        board.insert_word(&w);
    }
}

fn generate_word(gen: &mut impl WordGenerator, board: &Board) -> Word {
    let word = gen.draw_word();
    println!("Sua palavra é: {}", word);
    let orientation = DefaultGenerator::draw_orientation();
    board.create_word(word, orientation)
}

pub fn init_generator(words: Array) -> DefaultGenerator {
    let words_set = words.map(&mut |w, _, _| w.as_string().unwrap()).collect();
    let gen = DefaultGenerator::new(words_set);
    gen
}

#[wasm_bindgen]
pub fn read_sz() -> u32 {
    println!("No board size were given as arguments. Please specify it below:");
    let mut sz = String::new();
    stdin().read_line(&mut sz).expect("Something went wrong during input");
    let size = sz.trim().parse::<u32>().expect("Expected board size");
    size
}

pub fn fill_board(mut board: &mut Board, words: Array) {
    let vector = words.map(&mut |v, _, _| JsValue::as_string(v).unwrap()).collect();
    let gen = DefaultGenerator::new(&vector);
    place_words(&mut board, vector, &gen);
    board.fill_blank_slots(gen);
}