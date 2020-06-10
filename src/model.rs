use rand::Rng;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub enum Orientation {
    VERT,
    HOR,
}

pub struct Word {
    value: String,
    pub start_pos: (usize, usize),
    pub orientation: Orientation
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
    pub fn new(size: usize) -> Board{
        return Board {elements: vec![vec![String::from("□"); size]; size]}
    }

    pub fn create_word(self: &Self, word: String, orient: Orientation) -> Word {
        return Word {
            value: word.clone(),
            orientation: orient,
            start_pos: self.gen_valid_pos(&word, &orient)
        }
    }

    pub fn gen_valid_pos(self: &Self, word: &String, orient: &Orientation) -> (usize, usize) {
        let board_size = self.elements.len();
        let start_pos: (usize, usize) = (rand::thread_rng().gen_range(0, board_size - word.chars().count()), rand::thread_rng().gen_range(0, board_size));

        return match orient {
            Orientation::HOR => start_pos,
            Orientation::VERT => (start_pos.1, start_pos.0)
        };
    }

    pub fn insert_word(self: &mut Self, word: &Word){
        for idx in 0..word.value.chars().count() {
            match word.orientation {
                Orientation::HOR => { self.elements[word.start_pos.0 + idx][word.start_pos.1] = word.value.chars().nth(idx).unwrap().to_string() },
                Orientation::VERT => { self.elements[word.start_pos.0][word.start_pos.1 + idx] = word.value.chars().nth(idx).unwrap().to_string() }
            }
        }
    }
}