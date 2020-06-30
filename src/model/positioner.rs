use rand::Rng;

use super::*;

pub trait Positioner {
    fn gen_valid_pos(self: &Self, word: &String, orient: &Orientation) -> (usize, usize);
    fn has_collision(self: &Self, word: &String, orient: &Orientation, position: (usize, usize)) -> bool;
}

impl Positioner for Board {
    fn gen_valid_pos(self: &Self, word: &String, orient: &Orientation) -> (usize, usize) {
        let board_size = self.size;
        let mut position;

        while {
            let word_len = word.chars().count();
            if board_size < word_len {
                panic!("Board size doesn't fit word with {} chars", word_len)
            }
            let start_pos: (usize, usize) = (rand::thread_rng().gen_range(0, board_size - word_len), rand::thread_rng().gen_range(0, board_size));

            position = match orient {
                Orientation::HOR => start_pos,
                Orientation::VERT => (start_pos.1, start_pos.0)
            };

            self.has_collision(word, orient, position)
        } {}
        position
    }

    fn has_collision(self: &Self, word: &String, orient: &Orientation, position: (usize, usize)) -> bool {
        for (i, v) in word.chars().into_iter().enumerate() {
            let pos = self.get_oriented_pos(position, orient, i);
            let elem = &self.elements[pos];
            if *elem != Board::default_symbol() && *elem != v.to_string() {
                return true;
            }
        }
        false
    }
}