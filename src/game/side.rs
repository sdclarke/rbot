use crate::game::side::Side::{North, South};

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn index(&self) -> usize {
        match self {
            South => 0,
            North => 1,
        }
    }
}
