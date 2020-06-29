mod generator;
mod model;
mod ui;

#[wasm_bindgen]
pub fn main() {
    //let mut jsboard = model::wasm::JsBoard::with_capacity(read_sz());
    let mut board = model::Board::new(read_sz());

    let words: Array = vec!["fadoa", "amora", "feliz", "espirro", "mamacita", "victor", "beijo"].iter().map(|v| JsValue::from_str(v)).collect();


    fill_board(&mut board, words);

    println!("{}", board)
}


