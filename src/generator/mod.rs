use rand::Rng;

use word_set::TakeRandom;

use crate::generator::word_set::TakeRandomChar;
use crate::model::Orientation;

mod word_set;

pub trait WordGenerator {
    fn draw_orientation() -> Orientation;
    fn draw_word(&mut self) -> String;
}

pub trait CharGenerator {
    fn take_random_char(&self) -> char;
}

#[derive(Clone)]
pub struct DefaultGenerator {
    grammar: Vec<String>
}

impl WordGenerator for DefaultGenerator {
    fn draw_orientation() -> Orientation {
        match rand::thread_rng().gen_range(0, 2) {
            0 => Orientation::HOR,
            _ => Orientation::VERT
        }
    }

    fn draw_word(&mut self) -> String {
        self.grammar.remove(self.grammar.take_random_idx())
    }
}

impl CharGenerator for DefaultGenerator {
    fn take_random_char(&self) -> char {
        let word = self.grammar.take_random();
        word.take_random()
    }
}

impl DefaultGenerator {
    pub fn new(words: Vec<String>) -> DefaultGenerator {
        DefaultGenerator { grammar: words }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_random_char() {
        let words_set = vec!["Hello".to_string(), "World".to_string()];
        let generator = DefaultGenerator { grammar: words_set };
        let expected = generator.take_random_char();
        assert!(contains_char(&generator.grammar, expected));
    }

    fn contains_char(words_set: &Vec<String>, expected: char) -> bool {
        for word in words_set {
            for char_ in word.chars() {
                if char_ == expected {
                    return true;
                }
            }
        }
        false
    }
}