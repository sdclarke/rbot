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

    fn get_possible_moves(&self, board: &Board, side: Side) -> Result<Vec<HeuristicValue>, &'static str> {
        let mut h: Vec<HeuristicValue> = Vec::new();
        for i in 1..board.get_no_of_holes()+1 {
            if board.is_legal(side, i) {
                let heuristic = self.move_heuristic(board, side, i)?;
                h.push(heuristic);
            }
        }
        Ok(h)
    }

    fn move_heuristic(&self, board: &Board, side: Side, hole: usize) -> Result<HeuristicValue, &'static str> {
        let mut score: i64 = 0;
        let mut clone = board.clone();
        let next_side = clone.make_move(side, hole)?;
        if next_side == self.side {
            score += 10;
        } else {
            score -= 10;
        }
        let board_heuristic = self.board_heuristic(clone, next_side)?;
        score += board_heuristic;
        Ok(HeuristicValue{
            hole,
            heuristic: score,
        })
    }

    fn board_heuristic(&self, board: &Board, side: Side) -> Result<i64, &'static str> {
        let mut score: i64 = board.get_seeds_in_store(self.side) - board.get_seeds_in_store(self.side.opposite());
        let mut seeds_on_side: i64 = 0;
        for i in 5..board.get_no_of_holes()+1 {
            seeds_on_side += board.get_seeds(self.side, i)?;
        }
        score += seeds_on_side / 8;
        if board.get_seeds_in_store(self.side) > 49 {
            score += 100;
        } else if board.get_seeds_in_store(self.side.opposite()) > 49 {
            score -= 100;
        }
        let mut max_steal: i64 = 0;
        for i in 1..board.get_no_of_holes()+1 {
            let seeds = board.get_seeds(side, i)?;
    }
}

#[derive(Debug)]
struct HeuristicValue {
    hole: usize,
    heuristic: i64,
}
