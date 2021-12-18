use crate::game::board::Board;
use crate::game::side::Side;

#[derive(Debug)]
pub struct Minimax {
    side: Side
}

pub fn new_minimax(side: Side) -> Minimax {
    Minimax {
        side,
    }
}

impl Minimax {
    pub fn update_side(&mut self, side: Side) {
        self.side = side;
    }

    pub fn get_best_move(&self, board: &Board) -> Result<usize, &'static str> {
        let mut max = i32::MAX;
        let mut max_hole: usize = 0;
        Ok(max_hole)
    }
}
