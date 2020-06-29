use std::fmt;
use std::fmt::Formatter;

use js_sys::*;
use wasm_bindgen::*;
use wasm_bindgen::prelude::*;

pub use positioner::Positioner;
pub use word_factory::WordFactory;

use crate::generator::CharGenerator;

mod positioner;
mod word_factory;

#[wasm_bindgen]
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
    pub size: u32,
    pub elements: Array,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{}", self.elements.get(self.get_pos(i, j)).as_string().unwrap()).ok();
            }
            writeln!(f, "").ok();
        }
        Ok(())
    }
}

impl Board {
    fn default_symbol() -> JsValue {
        JsValue::from_str("□")
    }

    pub fn new(size: u32) -> Board {
        let elements = Array::new_with_length(size * size).iter().map(|_v| Board::default_symbol()).collect();
        return Board { size, elements };
    }

    fn get_pos(self: &Self, x: u32, y: u32) -> u32 {
        let offset = x * self.size;
        y + offset
    }
    fn get_oriented_pos(self: &Self, x: u32, y: u32, orient: Orientation, idx: u32) -> u32 {
        match orient {
            Orientation::HOR => self.get_pos(x + (idx as u32), y),
            Orientation::VERT => self.get_pos(x, y + (idx as u32))
        }
    }
    pub fn insert_word(self: &mut Self, word: &Word) {
        let (x, y) = word.start_pos;
        let word_value = JsString::from(word.value.as_str());
        for idx in 0..word_value.length() as u32 {
            let pos = self.get_oriented_pos(x as u32, y as u32, word.orientation, idx);
            self.elements.set(pos, JsValue::from(word_value.char_at(idx)))
        }
    }

    pub fn fill_blank_slots(&mut self, generator: impl CharGenerator) {
        let blank = Board::default_symbol();
        self.elements = self.elements.map(&mut |v, _, _| {
            if v == blank {
                JsValue::from(JsString::from(generator.take_random_char()))
            } else {
                v
            }
        });
    }
}