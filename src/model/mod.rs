use std::fmt;
use std::fmt::Formatter;

pub use positioner::Positioner;
pub use word_factory::WordFactory;

use crate::generator::CharGenerator;

mod positioner;
mod word_factory;

#[derive(Copy, Clone)]
pub enum Orientation {
    VERT,
    HOR,
}

pub struct Word {
    value: String,
    pub start_pos: (usize, usize),
    pub orientation: Orientation,
}

pub struct Board {
    pub elements: Vec<Vec<String>>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in &self.elements {
            for j in i {
                write!(f, "{}", j).ok();
            }
            writeln!(f, "").ok();
        }
        Ok(())
    }
}

impl Board {
    fn default_symbol() -> String {
        String::from("□")
    }
    pub fn new(size: usize) -> Board {
        return Board { elements: vec![vec![Board::default_symbol(); size]; size] };
    }

    pub fn insert_word(self: &mut Self, word: &Word) {
        for idx in 0..word.value.chars().count() {
            match word.orientation {
                Orientation::HOR => { self.elements[word.start_pos.0 + idx][word.start_pos.1] = word.value.chars().nth(idx).unwrap().to_string() }
                Orientation::VERT => { self.elements[word.start_pos.0][word.start_pos.1 + idx] = word.value.chars().nth(idx).unwrap().to_string() }
            }
        }
    }

    pub fn fill_blank_slots(&mut self, generator: impl CharGenerator) {
        for line in self.elements.iter_mut() {
            for element in line.iter_mut() {
                if *element == Board::default_symbol() {
                    *element = generator.take_random_char().to_string()
                }
            }
        }
    }
}