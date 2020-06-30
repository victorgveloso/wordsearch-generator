use std::fmt;
use std::fmt::Formatter;

use serde_derive::*;

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

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub size: usize,
    pub elements: Vec<String>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{}", self.elements[self.get_pos(i, j)])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Board {
    fn default_symbol() -> String {
        String::from("□")
    }
    pub fn new(size: usize) -> Board {
        return Board { size, elements: vec![Board::default_symbol(); size * size] };
    }
    fn get_pos(self: &Self, x: usize, y: usize) -> usize {
        let offset = x * self.size;
        y + offset
    }
    fn get_oriented_pos(self: &Self, coord: (usize, usize), orient: &Orientation, idx: usize) -> usize {
        match orient {
            Orientation::HOR => self.get_pos(coord.0 + idx, coord.1),
            Orientation::VERT => self.get_pos(coord.0, coord.1 + idx)
        }
    }

    pub fn insert_word(self: &mut Self, word: &Word) {
        for idx in 0..word.value.chars().count() {
            let pos = self.get_oriented_pos(word.start_pos, &word.orientation, idx);
            self.elements[pos] = word.value.chars().nth(idx).unwrap().to_string();
        }
    }

    pub fn fill_blank_slots(&mut self, generator: &impl CharGenerator) {
        for element in self.elements.iter_mut() {
            if *element == Board::default_symbol() {
                *element = generator.take_random_char().to_string()
            }
        }
    }
}