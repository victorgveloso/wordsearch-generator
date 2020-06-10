use crate::model::Orientation;
use rand::Rng;

pub fn draw_orientation() -> Orientation {
    return match rand::thread_rng().gen_range(0, 2) {
        0 => Orientation::HOR,
        _ => Orientation::VERT
    };
}

pub fn draw_word() -> String {
    let words_set = ["fadoa", "amora", "feliz", "espirro", "mamacita", "victor", "beijo"];
    let word_idx = rand::thread_rng().gen_range(0, words_set.len());
    return words_set[word_idx].to_string();
}
