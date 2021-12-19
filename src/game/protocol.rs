use crate::game::board::Board;
use crate::game::protocol::MessageType::*;
use crate::game::protocol::MoveTurn::Move;
use crate::game::protocol::MoveTurn::MoveEnd;
use crate::game::side::Side::{North, South};

#[derive(Debug)]
pub enum MessageType {
    Start,
    State,
    End,
}

#[derive(Debug)]
pub enum MoveTurn {
    MoveEnd,
    Move(i8, bool),
}

pub fn get_message_type(message: &str) -> Result<MessageType, &'static str> {
    if message.starts_with("START") {
        return Ok(Start);
    } else if message.starts_with("CHANGE") {
        return Ok(State);
    } else if message.starts_with("END") {
        return Ok(End);
    }
    Err("Unknown message type")
}

pub fn interpret_start_message(message: &str) -> Result<bool, &'static str> {
    if message.ends_with("South") {
        return Ok(true);
    } else if message.ends_with("North") {
        return Ok(false);
    }
    Err("Unknown start message format")
}

pub fn interpret_state_message(message: &str, board: &mut Board) -> Result<MoveTurn, &'static str> {
    let m: i8;
    let s: Vec<&str> = message.split(';').collect();
    if s.len() != 4 {
        return Err("Incorrect state message");
    }
    if s[1] == "SWAP" {
        m = -1;
    } else {
        m = s[1].parse::<i8>().unwrap();
    }

    let board_parts: Vec<&str> = s[2].split(',').collect();
    if board_parts.len() != 2 * (board.get_no_of_holes() + 1) {
        return Err("Incorrect length of board in state message");
    }
    for (i, part) in board_parts.iter().enumerate().take(board.get_no_of_holes()) {
        board.set_seeds(North, i + 1, part.parse::<u8>().unwrap())?;
    }
    board.set_seeds_in_store(
        North,
        board_parts[board.get_no_of_holes()].parse::<u8>().unwrap(),
    );
    for i in 0..board.get_no_of_holes() {
        board.set_seeds(
            South,
            i + 1,
            board_parts[i + board.get_no_of_holes() + 1]
                .parse::<u8>()
                .unwrap(),
        )?;
    }
    board.set_seeds_in_store(
        South,
        board_parts[2 * board.get_no_of_holes() + 1]
            .parse::<u8>()
            .unwrap(),
    );
    let move_turn = match s[3] {
        "YOU" => Move(m, true),
        "OPP" => Move(m, false),
        "END" => MoveEnd,
        _ => return Err("Incorrect end of state message"),
    };
    Ok(move_turn)
}

pub fn create_move_message(hole: usize) -> String {
    format!("MOVE;{}\n", hole)
}

pub fn create_swap_message() -> String {
    "SWAP\n".to_string()
}
