use crate::game::side::Side;
use crate::game::side::Side::{North, South};
use array2d::Array2D;

#[derive(Debug)]
pub struct Board {
    holes: usize,
    state: Array2D<u8>,
}

pub fn new_board(holes: usize, seeds: u8) -> Board {
    let mut b = Board {
        holes,
        state: Array2D::filled_with(seeds, 2, holes + 1),
    };
    b.state[(0, 0)] = 0;
    b.state[(1, 0)] = 0;
    b
}

impl Board {
    pub fn get_no_of_holes(&self) -> usize {
        self.holes
    }

    pub fn set_seeds(&mut self, side: Side, hole: usize, seeds: u8) -> Result<(), &'static str> {
        if hole < 1 || hole > self.holes {
            return Err("Invalid hole number");
        }
        self.state[(side.index(), hole)] = seeds;
        Ok(())
    }

    pub fn set_seeds_op(&mut self, side: Side, hole: usize, seeds: u8) -> Result<(), &'static str> {
        if hole < 1 || hole > self.holes {
            return Err("Invalid hole number");
        }
        self.state[(side.opposite().index(), self.holes - hole + 1)] = seeds;
        Ok(())
    }

    pub fn get_seeds(&self, side: Side, hole: usize) -> Result<u8, &'static str> {
        if hole < 1 || hole > self.holes {
            return Err("Invalid hole number");
        }
        Ok(self.state[(side.index(), hole)])
    }

    pub fn get_seeds_op(&self, side: Side, hole: usize) -> Result<u8, &'static str> {
        if hole < 1 || hole > self.holes {
            return Err("Invalid hole number");
        }
        Ok(self.state[(side.opposite().index(), self.holes - hole + 1)])
    }

    pub fn set_seeds_in_store(&mut self, side: Side, seeds: u8) {
        self.state[(side.index(), 0)] = seeds;
    }

    pub fn get_seeds_in_store(&self, side: Side) -> u8 {
        self.state[(side.index(), 0)]
    }

    pub fn game_over(&self) -> bool {
        self.holes_empty(South) || self.holes_empty(North)
    }

    fn holes_empty(&self, side: Side) -> bool {
        for i in 1..self.holes + 1 {
            if self.state[(side.index(), i)] != 0 {
                return false;
            }
        }
        true
    }

    pub fn is_legal(&self, side: Side, hole: usize) -> bool {
        if hole < 1 || hole > self.holes {
            return false;
        }
        if self.state[(side.index(), hole)] < 1 {
            return false;
        }
        true
    }

    pub fn clone(&self) -> Board {
        Board {
            holes: self.holes,
            state: self.state.clone(),
        }
    }

    pub fn make_move(&mut self, side: Side, hole: usize) -> Result<Side, &'static str> {
        let seeds_to_sow = self.get_seeds(side, hole)?;
        self.set_seeds(side, hole, 0)?;
        let receiving_pits = 2 * self.holes + 1;
        let rounds = seeds_to_sow / u8::try_from(receiving_pits).unwrap();
        let mut extra = seeds_to_sow % u8::try_from(receiving_pits).unwrap();
        if rounds > 0 {
            for i in 1..self.holes + 1 {
                self.state[(South.index(), i)] += rounds;
                self.state[(North.index(), i)] += rounds;
            }
            self.state[(side.index(), 0)] += rounds;
        }

        let mut sow_side = side;
        let mut sow_hole = hole;
        while extra > 0 {
            sow_hole += 1;
            if sow_hole == 1 {
                sow_side = sow_side.opposite();
            }
            if sow_hole > self.holes {
                if sow_side == side {
                    sow_hole = 0;
                    self.state[(side.index(), 0)] += 1;
                    extra -= 1;
                    continue;
                } else {
                    sow_side = sow_side.opposite();
                    sow_hole = 1;
                }
            }
            self.state[(sow_side.index(), sow_hole)] += 1;
            extra -= 1;
        }

        if sow_side == side && sow_hole > 0 {
            let sow_hole_seeds = self.get_seeds(sow_side, sow_hole)?;
            let sow_hole_seeds_op = self.get_seeds_op(sow_side, sow_hole)?;
            if sow_hole_seeds == 1 && sow_hole_seeds_op > 0 {
                self.state[(side.index(), 0)] += 1 + sow_hole_seeds_op;
                self.set_seeds(side, sow_hole, 0)?;
                self.set_seeds_op(side, sow_hole, 0)?;
            }
        }
        let mut finished_side: Option<Side> = None;
        if self.holes_empty(side) {
            finished_side = Some(side);
        } else if self.holes_empty(side.opposite()) {
            finished_side = Some(side.opposite());
        }
        if let Some(finished) = finished_side {
            let mut seeds = 0;
            let collecting_side = finished.opposite();
            for i in 1..self.holes + 1 {
                seeds += self.state[(collecting_side.index(), i)];
                self.set_seeds(collecting_side, i, 0)?;
            }
            self.state[(collecting_side.index(), 0)] += seeds;
        }
        if sow_hole == 0 {
            return Ok(side);
        }
        Ok(side.opposite())
    }
}
