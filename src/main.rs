mod game;
use std::io;

fn send_message(msg: &str) {
    print!("{}", msg);
}

fn recv_message() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.pop();
    Ok(input)
}

fn main() {
    let mut b = game::board::new_board(7, 7);
    let mut side = game::side::Side::South;
    let mut can_swap = true;
    let mut minimax = game::minimax::new_minimax(side);

    loop {
        let message = recv_message().unwrap();
        let message_type = game::protocol::get_message_type(&message).unwrap();
        match message_type {
            game::protocol::MessageType::Start => {
                let south = game::protocol::interpret_start_message(&message).unwrap();
                if south {
                    can_swap = false;
                    send_message(&game::protocol::create_move_message(1));
                } else {
                    side = side.opposite();
                    minimax.update_side(side);
                }
            }
            game::protocol::MessageType::State => {
                let move_turn = game::protocol::interpret_state_message(&message, &mut b).unwrap();
                match move_turn {
                    game::protocol::MoveTurn::MoveEnd => return,
                    game::protocol::MoveTurn::Move(hole, again) => {
                        if hole == -1 {
                            side = side.opposite();
                            minimax.update_side(side);
                        }
                        if again {
                            if can_swap {
                                side = side.opposite();
                                minimax.update_side(side);
                                send_message(&game::protocol::create_swap_message());
                            } else {
                                let best_hole = minimax.get_best_move(&b).unwrap();
                                send_message(&game::protocol::create_move_message(best_hole));
                            }
                            can_swap = false;
                        }
                    }
                }
            }
            game::protocol::MessageType::End => return,
        }
    }
}
