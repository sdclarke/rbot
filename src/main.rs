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
        let message_type = game::protocol::get_message_type(&message);
        match message_type {
            game::protocol::MessageType::Start => {
                println!("{}", message);
                let south = game::protocol::interpret_start_message(&message);
                if south {
                    can_swap = false;
                    send_message("MOVE MESSAGE HERE\n");
                } else {
                    side = side.opposite();
                    minimax.update_side(side);
                }
                println!("{:?}", minimax);
            }
            _ => println!("Not Start"),
        }
        break;
    }
}
