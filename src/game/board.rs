use crate::game::side::Side;
use crate::game::side::Side::{South, North};
use array2d::Array2D;

#[derive(Debug)]
pub struct Board {
    holes: usize,
    state: Array2D<u8>,
}

pub fn new_board(holes: usize, seeds: u8) -> Board {
    let mut b = Board {
        holes: holes,
        state: Array2D::filled_with(seeds, 2, holes+1),
    };
    b.state[(0, 0)] = 0;
    b.state[(1, 0)] = 0;
    b
}

impl Board {
    pub fn get_no_of_holes(&self) -> usize {
        self.holes
    }

    pub fn set_seeds(&mut self, side: Side, hole: usize, seeds: u8) {
        match side {
            South => self.state[(0, hole)] = seeds,
            North => self.state[(1, hole)] = seeds,
        }
    }
}
