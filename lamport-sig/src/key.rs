use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Key {
    pub zeros: Vec<String>,
    pub ones: Vec<String>,
}

impl Key {
    pub fn new() -> Self {
        Self {
            zeros: Vec::with_capacity(256),
            ones: Vec::with_capacity(256),
        }
    }
}

impl Index<u8> for Key {
    type Output = Vec<String>;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.zeros,
            1 => &self.ones,
            _ => panic!("Index out of bounds: only 0 and 1 are valid bits."),
        }
    }
}

impl  IndexMut<u8> for Key {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.zeros,
            1 => &mut self.ones,
            _ => panic!("Index out of bounds: only 0 and 1 are valid bits."),
        }
    }
}