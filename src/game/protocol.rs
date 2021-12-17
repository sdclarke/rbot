//use crate::game::protocol::MessageType;
use crate::game::protocol::MessageType::*;

#[derive(Debug)]
pub enum MessageType {
    Start,
    State,
    End,
    Error,
}

pub fn get_message_type(message: &str) -> MessageType {
    if message.starts_with("START") {
        return Start;
    } else if message.starts_with("CHANGE") {
        return State;
    } else if message.starts_with("END") {
        return End;
    }
    return Error;
}

pub fn interpret_start_message(message: &str) -> bool {
    if message.ends_with("South") {
        return true;
    } else {
        return false;
    }
}
