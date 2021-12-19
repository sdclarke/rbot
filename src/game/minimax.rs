use crate::game::board::Board;
use crate::game::side::Side;

const MAX_DEPTH: i8 = 13;

#[derive(Debug)]
pub struct Minimax {
    side: Side,
}

pub fn new_minimax(side: Side) -> Minimax {
    Minimax { side }
}

impl Minimax {
    pub fn update_side(&mut self, side: Side) {
        self.side = side;
    }

    pub fn get_best_move(&self, board: &Board) -> Result<usize, &'static str> {
        let mut max = i64::MAX;
        let mut max_hole: usize = 0;
        let mut heuristic_values = self.get_possible_moves(board, self.side)?;
        heuristic_values.sort_by(|a, b| b.heuristic.cmp(&a.heuristic));
        let mut alpha = i64::MIN;
        let beta = i64::MAX;
        for node in heuristic_values {
            let mut clone = board.clone();
            let next_side = clone.make_move(self.side, node.hole)?;
            let score = self.do_minimax(
                &clone,
                next_side,
                next_side == self.side,
                MAX_DEPTH,
                alpha,
                beta,
            )?;
            if score > max {
                max = score;
                max_hole = node.hole;
                if max > alpha {
                    alpha = max
                }
            }
        }
        Ok(max_hole)
    }

    fn do_minimax(
        &self,
        board: &Board,
        side: Side,
        max: bool,
        depth: i8,
        mut alpha: i64,
        mut beta: i64,
    ) -> Result<i64, &'static str> {
        if depth == 0 || board.game_over() {
            return self.board_heuristic(board, side);
        }
        let mut max_score = i64::MIN;
        let mut min_score = i64::MAX;
        let mut heuristic_values = self.get_possible_moves(board, side)?;
        if max {
            heuristic_values.sort_by(|a, b| b.heuristic.cmp(&a.heuristic));
        } else {
            heuristic_values.sort_by(|a, b| a.heuristic.cmp(&b.heuristic));
        }
        for node in heuristic_values {
            let mut clone = board.clone();
            let next_side = clone.make_move(side, node.hole)?;
            let score = self.do_minimax(
                &clone,
                next_side,
                next_side == self.side,
                depth - 1,
                alpha,
                beta,
            )?;
            if max && score > max_score {
                max_score = score;
                if max_score > alpha {
                    alpha = max_score
                }
            } else if !max && score < min_score {
                min_score = score;
                if min_score < beta {
                    beta = min_score;
                }
            }
            if beta <= alpha {
                if max {
                    return Ok(max_score);
                }
                return Ok(min_score);
            }
        }
        if max {
            return Ok(max_score);
        }
        Ok(min_score)
    }

    fn get_possible_moves(
        &self,
        board: &Board,
        side: Side,
    ) -> Result<Vec<HeuristicValue>, &'static str> {
        let mut h: Vec<HeuristicValue> = Vec::new();
        for i in 1..board.get_no_of_holes() + 1 {
            if board.is_legal(side, i) {
                let heuristic = self.move_heuristic(board, side, i)?;
                h.push(heuristic);
            }
        }
        Ok(h)
    }

    fn move_heuristic(
        &self,
        board: &Board,
        side: Side,
        hole: usize,
    ) -> Result<HeuristicValue, &'static str> {
        let mut score: i64 = 0;
        let mut clone = board.clone();
        let next_side = clone.make_move(side, hole)?;
        if next_side == self.side {
            score += 10;
        } else {
            score -= 10;
        }
        let board_heuristic = self.board_heuristic(&clone, next_side)?;
        score += board_heuristic;
        Ok(HeuristicValue {
            hole,
            heuristic: score,
        })
    }

    fn board_heuristic(&self, board: &Board, side: Side) -> Result<i64, &'static str> {
        let mut score: i64 = (board.get_seeds_in_store(self.side)
            - board.get_seeds_in_store(self.side.opposite()))
        .into();
        let mut seeds_on_side: i64 = 0;
        for i in 5..board.get_no_of_holes() + 1 {
            seeds_on_side += i64::from(board.get_seeds(self.side, i)?);
        }
        score += seeds_on_side / 8;
        if board.get_seeds_in_store(self.side) > 49 {
            score += 100;
        } else if board.get_seeds_in_store(self.side.opposite()) > 49 {
            score -= 100;
        }
        let mut max_steal: i64 = 0;
        for i in 1..board.get_no_of_holes() + 1 {
            let seeds = board.get_seeds(side, i)?;
            let land = (i + usize::from(seeds)) % 15;
            if land > 0 && land <= 7 {
                let land_seeds = board.get_seeds(side, land)?;
                if seeds > 0 && seeds <= 15 && (land_seeds == 0 || (seeds == 15 && land == i)) {
                    let seeds_op = board.get_seeds_op(side, land)?;
                    if seeds_op > 0 && max_steal < (seeds_op + 1).into() {
                        max_steal = (seeds_op + 1).into();
                    }
                }
            }
        }
        if side == self.side {
            score += max_steal;
        } else {
            score -= max_steal;
        }
        Ok(score)
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HeuristicValue {
    hole: usize,
    heuristic: i64,
}
