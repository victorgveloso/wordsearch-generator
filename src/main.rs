use std::io::stdin;

use generator::DefaultGenerator;

use crate::generator::WordGenerator;
use crate::model::{Orientation, WordFactory};

mod generator;
mod model;

fn main() {
    let mut board = model::Board::new(read_sz());

    let gen = init_generator(vec!["fadoa", "amora", "feliz", "espirro", "mamacita", "victor", "beijo"]);

    for _ in 0..7 {
        let (word, orientation) = gen_oriented_word(&mut gen.clone());

        let w = board.create_word(word, orientation);

        println!("{}ª linha, {}ª coluna", w.start_pos.0 + 1, w.start_pos.1 + 1);

        board.insert_word(&w);
    }

    board.fill_blank_slots(gen);

    println!("{}", board)
}

fn gen_oriented_word(gen: &mut impl WordGenerator) -> (String, Orientation) {
    let word = gen.draw_word();
    println!("Sua palavra é: {}", word);
    let orientation = DefaultGenerator::draw_orientation();
    (word, orientation)
}

fn init_generator(words: Vec<&str>) -> DefaultGenerator {
    let words_set = words.iter().map(|s| s.to_string()).collect();
    let gen = DefaultGenerator::new(words_set);
    gen
}

fn read_sz() -> usize {
    println!("No board size were given as arguments. Please specify it below:");
    let mut sz = String::new();
    stdin().read_line(&mut sz).expect("Something went wrong during input");
    let size = sz.trim().parse::<usize>().expect("Expected board size");
    size
}
