mod generator;
mod model;
mod ui;
use ui::{read_sz, place_words, init_generator};


fn main() {
    let mut board = model::Board::new(read_sz());

    let words = vec!["fadoa", "amora", "feliz", "espirro", "mamacita", "victor", "beijo"];
    let gen = init_generator(&words);

    place_words(&mut board, words, &gen);

    board.fill_blank_slots(gen);

    println!("{}", board)
}


