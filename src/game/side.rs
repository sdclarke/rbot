use crate::game::side::Side::{South, North};

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Side {
    South,
    North,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            South => North,
            North => South,
        }
    }
}
