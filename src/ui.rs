use std::io::stdin;
use crate::generator::*;
use crate::model::{WordFactory, Word, Board};

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

pub fn init_generator(words: &Vec<&str>) -> DefaultGenerator {
    let words_set = words.iter().map(ToString::to_string).collect();
    let gen = DefaultGenerator::new(words_set);
    gen
}

pub fn read_sz() -> usize {
    println!("No board size were given as arguments. Please specify it below:");
    let mut sz = String::new();
    stdin().read_line(&mut sz).expect("Something went wrong during input");
    let size = sz.trim().parse::<usize>().expect("Expected board size");
    size
}