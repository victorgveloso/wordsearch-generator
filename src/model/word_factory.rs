use super::*;
use super::positioner::Positioner;

pub trait WordFactory {
    fn create_word(self: &Self, word: String, orient: Orientation) -> Word;
}

impl WordFactory for Board {
    fn create_word(self: &Self, word: String, orient: Orientation) -> Word {
        Word {
            value: word.clone(),
            orientation: orient,
            start_pos: self.gen_valid_pos(&word, &orient),
        }
    }
}
