use std::vec::Vec;

use rand::Rng;

pub trait TakeRandom<T> {
    fn take_random_idx(&self) -> usize;
    fn take_random(&self) -> &T;
}

pub trait TakeRandomChar {
    fn take_random_idx(&self) -> usize;
    fn take_random(&self) -> char;
}

impl<T> TakeRandom<T> for Vec<T> {
    fn take_random_idx(&self) -> usize {
        rand::thread_rng().gen_range(0, self.len())
    }

    fn take_random(&self) -> &T {
        let word_idx = self.take_random_idx();
        &self[word_idx]
    }
}

impl TakeRandomChar for String {
    fn take_random_idx(&self) -> usize {
        rand::thread_rng().gen_range(0, self.len())
    }

    fn take_random(&self) -> char {
        let letter_idx = self.take_random_idx();
        self.chars().nth(letter_idx).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::generator::word_set::TakeRandomChar;

    use super::TakeRandom;

    #[test]
    fn take_one() {
        let vector: Vec<String> = vec!["AB", "BC", "CD", "DE", "EF", "FG", "GH"].iter().map(ToString::to_string).collect();
        let element = vector.take_random();
        let c = element.take_random();
        println!("{} from {}", c, element);
        assert!(vector.contains(element));
        assert!(element.contains(c));
    }
}