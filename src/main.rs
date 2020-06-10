mod generator;
mod model;
use std::io::stdin;
fn main() {
    let mut sz= String::new();

    stdin().read_line(&mut sz).expect("Something went wrong during input");

    let size = sz.trim().parse::<usize>().expect("Expected board size");

    let word = generator::draw_word();
    println!("Sua palavra é: {}", word);

    let mut board: model::Board = model::Board::new(size);

    let orientation = generator::draw_orientation();

    let w = board.create_word(word, orientation);

    println!("{}ª linha, {}ª coluna", w.start_pos.0 + 1, w.start_pos.1 + 1);

    board.insert_word(&w);

    println!("{}", board)
}
